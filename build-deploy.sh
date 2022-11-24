#!/bin/bash

./build.sh

docker build -f share.Dockerfile -t experiment:fft-in-the-browser .

docker tag experiment:fft-in-the-browser c0yote/experiment:fft-in-the-browser

docker push c0yote/experiment:fft-in-the-browser