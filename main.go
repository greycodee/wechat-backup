package main

import (
	"fmt"
	"log"
	"net/http"
)

func main() {

	http.Handle("/api/", route())
	err := http.ListenAndServe(":8080", nil)
	if err != nil {
		log.Fatal(err)
	}
}

func route() http.Handler {
	return &API{}
}

type API struct {
}

func (api *API) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	upath := r.URL.Path
	fmt.Println(upath)

	w.Write([]byte(upath))

}
