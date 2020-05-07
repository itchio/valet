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
//   char *value;
//   size_t len;
// } NString;
//
// typedef struct {
//   NString db_path;
//   int64_t id;
// } ServerOpts;
import "C"

//export ServerNew
func ServerNew(cOpts *C.ServerOpts) C.int {
	opts := server.NewOpts{
		DBPath: nstring(&cOpts.db_path),
	}

	id, err := server.New(opts)
	if err != nil {
		log.Printf("Could not create new server: %+v", err)
		return 1
	}

	cOpts.id = C.int64_t(id)
	return 0
}

//export ServerSend
func ServerSend(cId C.int64_t, cPayload C.NString) C.int {
	payload := C.GoBytes(unsafe.Pointer(cPayload.value), C.int(cPayload.len))
	server.Send(int64(cId), payload)
	return 0
}

//export ServerRecv
func ServerRecv(cId C.int64_t, cPayload *C.NString) C.int {
	payload := server.Recv(int64(cId))
	ptr := C.CBytes(payload)
	cPayload.value = (*C.char)(ptr)
	cPayload.len = C.size_t(len(payload))
	return 0
}

//export ServerFree
func ServerFree(cId C.int64_t) C.int {
	server.Free(int64(cId))
	return 0
}

//export NStringFree
func NStringFree(n *C.NString) {
	if n == nil {
		return
	}

	if n.value != nil {
		C.free(unsafe.Pointer(n.value))
		n.value = nil
	}
}

func nstring(n *C.NString) string {
	return C.GoStringN(n.value, C.int(n.len))
}

func main() {
	// sic. - required by cgo
}
