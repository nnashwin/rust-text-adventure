#!/bin/bash
wasm-pack build --target web && rollup -c
