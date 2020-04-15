package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
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

	var cres CountryResponse
	err = json.Unmarshal(body, &cres)
	must(err)

	fmt.Printf("You are in: %s\n", cres.Country)
}

func must(err error) {
	if err != nil {
		panic(fmt.Sprintf("%+v", err))
	}
}

func main() {
	// sic. - required by cgo
}

