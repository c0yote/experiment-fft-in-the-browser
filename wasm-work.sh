#!/bin/bash

docker build -t audio-fft:rust-build .

cd wasm-fft

docker run -it --rm -v ${PWD}:/work audio-fft:rust-build /bin/bash