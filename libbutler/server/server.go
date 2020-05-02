package server

import (
	"context"
	"fmt"
	"log"
	"net"
	"os"
	"path/filepath"
	"sync"

	"crawshaw.io/sqlite/sqlitex"
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
	transport *directTransport
	conn      jsonrpc2.Conn
}

type NewOpts struct {
	DBPath string
}

type ServerID = int64

type serverMap struct {
	lock *sync.RWMutex
	m    map[ServerID]*Server
	rand idMaker
}

var servers = serverMap{
	lock: new(sync.RWMutex),
	m:    make(map[ServerID]*Server),
	rand: newIdMaker(),
}

func New(opts NewOpts) (ServerID, error) {
	if opts.DBPath == "" {
		return 0, errors.New("DBPath cannot be nil")
	}

	listener, err := net.Listen("tcp", "127.0.0.1:")
	must(err)

	log.Printf("Listening on %s", listener.Addr().String())

	mansionCtx := mansion.NewContext(nil)
	mansionCtx.DBPath = opts.DBPath

	err = os.MkdirAll(filepath.Dir(mansionCtx.DBPath), 0o755)
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
	defer dbPool.Close()

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

	ctx := context.Background()

	transport := newDirectTransport()
	conn := jsonrpc2.NewConn(ctx, transport, router)

	s := &Server{
		conn:      conn,
		transport: transport,
	}
	servers.lock.Lock()
	id := servers.rand.next()
	servers.m[id] = s
	servers.lock.Unlock()
	return id, nil
}

func Free(id ServerID) {
	servers.lock.Lock()
	delete(servers.m, id)
	servers.lock.Unlock()
}

func must(err error) {
	if err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
}
