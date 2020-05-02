package server

import "github.com/itchio/butler/butlerd/jsonrpc2"

type directTransport struct {
	closed   bool
	incoming chan []byte
	outgoing chan []byte
}

var _ jsonrpc2.Transport = (*directTransport)(nil)

func newDirectTransport() *directTransport {
	return &directTransport{
		closed:   false,
		incoming: make(chan []byte, 128),
		outgoing: make(chan []byte, 128),
	}
}

func (dt *directTransport) Read() ([]byte, error) {
	msg := <-dt.incoming
	return msg, nil
}

func (dt *directTransport) Write(msg []byte) error {
	dt.outgoing <- msg
	return nil
}

func (dt *directTransport) Close() error {
	if dt.closed {
		return nil
	}
	dt.closed = true
	close(dt.incoming)
	close(dt.outgoing)
	return nil
}
