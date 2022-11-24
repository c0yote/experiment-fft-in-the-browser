#!/bin/bash

docker build -t audio-fft:rust-build .

docker run -it --rm -v ${PWD}/wasm-fft:/work audio-fft:rust-build wasm-pack build --release --target web

docker run -it --rm -v ${PWD}/fft-in-the-browser:/work -p 3000:3000 -w=/work node:19-bullseye npm install