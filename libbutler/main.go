package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"

	"github.com/itchio/valet/libbutler/server"
)

// #include <stdint.h>
//
// typedef struct {
//   char *value;
//   int len;
// } NString;
//
// typedef struct {
//   NString name;
//   int64_t id;
// } ServerOpts;
import "C"

type CountryResponse struct {
	Country string `json:"country"`
}

//export PrintCountry
func PrintCountry() {
	res, err := http.Get("https://itch.io/country")
	must(err)

	body, err := ioutil.ReadAll(res.Body)
	must(err)

	var cres CountryResponse
	err = json.Unmarshal(body, &cres)
	must(err)

	fmt.Printf("You are in: %s\n", cres.Country)

	doPanic()
}

//export ServerNew
func ServerNew(cOpts *C.ServerOpts) C.int {
	opts := server.NewOpts{
		DBPath: C.GoStringN(cOpts.name.value, cOpts.name.len),
	}

	id, err := server.New(opts)
	if err != nil {
		log.Printf("Could not create new server: %+v", err)
		return 1
	}

	cOpts.id = C.int64_t(id)
	return 0
}

func doPanic() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("Recovered: ", r)
		}
	}()

	panic("panic from go")
}

func must(err error) {
	if err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
}

func main() {
	// sic. - required by cgo
}
