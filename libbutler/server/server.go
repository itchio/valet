package server

import (
	"context"
	"os"
	"path/filepath"
	"sync"

	"crawshaw.io/sqlite/sqlitex"
	"github.com/itchio/butler/butlerd"
	"github.com/itchio/butler/butlerd/horror"
	"github.com/itchio/butler/butlerd/jsonrpc2"
	"github.com/itchio/butler/cmd/daemon"
	"github.com/itchio/butler/comm"
	"github.com/itchio/butler/database"
	"github.com/itchio/butler/mansion"
	"github.com/itchio/headway/state"
	"github.com/pkg/errors"
)

type Server struct {
	ctx    context.Context
	router *butlerd.Router
}

// singleton
var globalServer *Server

type Conn struct {
	transport *directTransport
	rpcConn   jsonrpc2.Conn
}

type InitOpts struct {
	DBPath    string
	UserAgent string
	Address   string
}

type ConnID = int64

type connMap struct {
	lock *sync.RWMutex
	m    map[ConnID]*Conn
	rand idMaker
}

var conns = connMap{
	lock: new(sync.RWMutex),
	m:    make(map[ConnID]*Conn),
	rand: newIdMaker(),
}

func Initialize(opts InitOpts) error {
	if opts.DBPath == "" {
		return errors.New("DBPath cannot be nil")
	}

	mansionCtx := mansion.NewContext(nil)
	if opts.Address == "" {
		mansionCtx.SetAddress(opts.Address)
	} else {
		mansionCtx.SetAddress("https://itch.io")
	}
	if opts.UserAgent != "" {
		mansionCtx.UserAgentAddition = opts.UserAgent
	}
	mansionCtx.DBPath = opts.DBPath

	err := os.MkdirAll(filepath.Dir(mansionCtx.DBPath), 0o755)
	if err != nil {
		mansionCtx.Must(errors.WithMessage(err, "creating DB directory if necessary"))
	}

	justCreated := false
	_, statErr := os.Stat(mansionCtx.DBPath)
	if statErr != nil {
		comm.Logf("butlerd: creating new DB at %s", mansionCtx.DBPath)
		justCreated = true
	}

	dbPool, err := sqlitex.Open(mansionCtx.DBPath, 0, 100)
	if err != nil {
		mansionCtx.Must(errors.WithMessage(err, "opening DB for the first time"))
	}

	err = func() (retErr error) {
		defer horror.RecoverInto(&retErr)

		conn := dbPool.Get(context.Background())
		defer dbPool.Put(conn)
		return database.Prepare(&state.Consumer{
			OnMessage: func(lvl string, msg string) {
				comm.Logf("[db prepare] [%s] %s", lvl, msg)
			},
		}, conn, justCreated)
	}()
	if err != nil {
		mansionCtx.Must(errors.WithMessage(err, "preparing DB"))
	}

	// TODO: don't use mansionCtx when building router
	router := daemon.GetRouter(dbPool, mansionCtx)

	globalServer = &Server{
		ctx:    context.Background(),
		router: router,
	}
	return nil
}

func EnsureGlobalServer() *Server {
	if globalServer == nil {
		panic("global server not running - initialize() must be called first!")
	}
	return globalServer
}

func ConnNew() ConnID {
	s := EnsureGlobalServer()

	transport := newDirectTransport()
	rpcConn := jsonrpc2.NewConn(s.ctx, transport, s.router)

	conn := &Conn{
		transport: transport,
		rpcConn:   rpcConn,
	}

	conns.lock.Lock()
	connID := ConnID(conns.rand.next())
	conns.m[connID] = conn
	conns.lock.Unlock()

	return connID
}

func ConnSend(id ConnID, payload []byte) {
	conns.lock.RLock()
	conn := conns.m[id]
	conns.lock.RUnlock()

	conn.transport.incoming <- payload
}

func ConnRecv(id ConnID) []byte {
	conns.lock.RLock()
	conn := conns.m[id]
	conns.lock.RUnlock()

	if conn == nil {
		return nil
	}
	return <-conn.transport.outgoing
}

func ConnClose(id ConnID) {
	conns.lock.Lock()
	conn := conns.m[id]
	delete(conns.m, id)
	conns.lock.Unlock()

	conn.rpcConn.Close()
}
