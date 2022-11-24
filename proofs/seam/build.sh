#!/bin/bash

docker build -t audio-fft:rust-build .

docker run -it --rm -v ${PWD}/test-wasm:/work audio-fft:rust-build wasm-pack build --release --target web