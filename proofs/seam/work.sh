#!/bin/bash

cd wasm-fft

docker build -t audio-fft:rust-build .

docker run -it --rm -v ${PWD}:/work audio-fft:rust-build /bin/bash