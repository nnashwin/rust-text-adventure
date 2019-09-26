package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
)

const PORT = "8000"

func addMimeToWasmFiles(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/wasm")

	wasmF, err := os.Open("./pkg/rust_text_adventure_bg.wasm")
	if err != nil {
		fmt.Println(err)
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer wasmF.Close()

	io.Copy(w, wasmF)
}

func main() {
	mux := http.NewServeMux()

	mux.Handle("/", http.FileServer(http.Dir("./")))

	mux.HandleFunc("/pkg/rust_text_adventure.wasm", addMimeToWasmFiles)

	log.Printf("Server running at port: %s", PORT)
	log.Fatal(http.ListenAndServe(":"+PORT, mux))
}
