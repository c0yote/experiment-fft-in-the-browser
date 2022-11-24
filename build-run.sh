#!/bin/bash

./build.sh

docker run -it --rm -v ${PWD}:/work -p 3000:3000 -w=/work/fft-in-the-browser node:19-bullseye npm start