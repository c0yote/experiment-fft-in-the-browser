# experiment-fft-in-the-browser

Running an audio FFT in WebAssembly and visualizing with WebGL

This is an weekend experiment; not production code. I offer no guarantees on anything.

`/fft-in-the-browser` is a typescript react app with a component that manages the lifecycle of an audio file's fft analysis and rendering.

`/wasm-fft` is a rust -> web assembly implementation of an framed fft. (there is no frame overlapping in this example)

`/proofs` contains the individual problems I was exploring broken apart into their constituent pieces.

## Try It

You have to options, build it yourself or pull the public docker image from docker hub.

You will need docker eitherway because I used it to control the build environment.

### Run It

### Build & Run It

1. Have Docker installed.
2. Run `./build-run.sh`.
3. Wait for build to complete and development server to start.
4. Browse to `http://localhost:3000`

This will launch build containers and then a container to run the development server.

A large wav file will load and the FFT and View options will become active.

## Other Stuff

There are two ways to access "dev shells". One for the react app, and one for the rust wasm library.

`./react-work.sh` dumps you into a shell inside a docker container that can be used to build and run the react app.

`./wasm-work.sh` dumps you into a shell inside a docker container that can be used to build the rust web assembly package.
