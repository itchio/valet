package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"

	"github.com/itchio/valet/libbutler/server"
)

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

//export StartServer
func StartServer() {
	server.Start()
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
