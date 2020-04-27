package server

import (
	"context"
	"fmt"
	"log"
	"net"
	"os"
	"path/filepath"

	"crawshaw.io/sqlite/sqlitex"
	"github.com/itchio/butler/butlerd"
	"github.com/itchio/butler/butlerd/horror"
	"github.com/itchio/butler/cmd/daemon"
	"github.com/itchio/butler/comm"
	"github.com/itchio/butler/database"
	"github.com/itchio/butler/mansion"
	"github.com/itchio/headway/state"
	"github.com/pkg/errors"
)

func Start() {
	secret := "foobar"
	s := butlerd.NewServer(secret)

	consumer := &state.Consumer{}

	listener, err := net.Listen("tcp", "127.0.0.1:")
	must(err)

	log.Printf("Listening on %s", listener.Addr().String())

	shutdownChan := make(chan struct{})

	mansionCtx := mansion.NewContext(nil)
	// FIXME: just testing
	mansionCtx.DBPath = "c:/Users/amos/AppData/Roaming/kitch/db/butler.db"
	mansionCtx.EnsureDBPath()

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

	router := daemon.GetRouter(dbPool, mansionCtx)

	ctx := context.Background()
	go func() {
		err = s.ServeTCP(ctx, butlerd.ServeTCPParams{
			Handler:  router,
			Consumer: consumer,
			Listener: listener,
			Secret:   secret,

			ShutdownChan: shutdownChan,
		})
		must(err)
	}()
}

func must(err error) {
	if err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
}
