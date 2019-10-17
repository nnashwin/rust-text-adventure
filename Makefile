all: pack run

.PHONY: clean

clean:
	rm -rf pkg target

pack:
	wasm-pack build --target web && rollup -c

run:
	go run server.go



