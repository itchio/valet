package main

import (
	"log"
	"unsafe"

	"github.com/itchio/valet/libbutler/server"
)

// #include <stdint.h>
// #include <stdlib.h>
//
// typedef struct {
//   uint8_t *value;
//   size_t len;
// } Buffer;
//
// typedef struct {
//   Buffer *db_path;
//   Buffer *user_agent;
//   Buffer *address;
// } InitOpts;
//
// typedef void (*recv_callback)(void *userdata, Buffer payload);
//
// static void call_recv_callback(recv_callback cb, void *userdata, Buffer payload) {
//   cb(userdata, payload);
// }
import "C"

//export butler_initialize
func butler_initialize(cOpts *C.InitOpts) C.int {
	opts := server.InitOpts{
		DBPath:    cOpts.db_path.ToString(),
		UserAgent: cOpts.user_agent.ToString(),
		Address:   cOpts.address.ToString(),
	}

	err := server.Initialize(opts)
	if err != nil {
		log.Printf("Could not create new server: %+v", err)
		return 1
	}

	return 0
}

//export butler_panic
func butler_panic() {
	panic("Panicking from go")
}

//export butler_conn_new
func butler_conn_new() C.int64_t {
	return C.int64_t(server.ConnNew())
}

//export butler_conn_send
func butler_conn_send(cId C.int64_t, cPayload *C.Buffer) C.int {
	server.ConnSend(int64(cId), cPayload.ToBytes())
	return 0
}

//export butler_conn_recv
func butler_conn_recv(cId C.int64_t, cb C.recv_callback, userdata unsafe.Pointer) {
	go func() {
		payload := server.ConnRecv(int64(cId))
		cPayload := C.Buffer{
			value: (*C.uint8_t)(unsafe.Pointer(C.CBytes(payload))),
			len:   C.size_t(len(payload)),
		}
		C.call_recv_callback(cb, userdata, cPayload)
	}()
}

//export butler_conn_close
func butler_conn_close(cId C.int64_t) C.int {
	server.ConnClose(int64(cId))
	return 0
}

//export butler_buffer_free
func butler_buffer_free(b *C.Buffer) {
	if unsafe.Pointer(b.value) != nil {
		C.free(unsafe.Pointer(b.value))
		b.value = nil
	}
}

func (b *C.Buffer) ToString() string {
	if unsafe.Pointer(b) == nil {
		return ""
	} else {
		return C.GoStringN((*C.char)(unsafe.Pointer(b.value)), C.int(b.len))
	}
}

func (b *C.Buffer) ToBytes() []byte {
	if unsafe.Pointer(b) == nil {
		return nil
	} else {
		return C.GoBytes(unsafe.Pointer(b.value), C.int(b.len))
	}
}

func main() {
	// sic. - required by cgo
}
