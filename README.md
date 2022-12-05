# experiment-fft-in-the-browser

Running an audio FFT in WebAssembly and visualizing with WebGL

![Screenshot](proofs/screenshot.png?raw=true "Title")

You can pull a docker image and run it locally if you want.

1. Run `docker run -it --rm -p 3000:3000 -w=/work/fft-in-the-browser c0yote/experiment:fft-in-the-browser npm start`
2. Wait for the development server to start.
3. Browse to `http://localhost:3000`.
4. A large wav file will load and the FFT and View options will become active.

## Notes

This is a weekend experiment; not production code. I offer no guarantees on anything.

`/fft-in-the-browser` is a typescript react app with a component that manages the lifecycle of an audio file's fft analysis and rendering.

`/wasm-fft` is a rust -> web assembly implementation of an framed fft analysis. (there is no frame overlapping in this example)

`/proofs` contains the individual problems I was exploring, broken apart into their constituent pieces.

## Other Stuff

There are two ways to access "dev shells". One for the react app, and one for the rust wasm library.

`./react-work.sh` dumps you into a shell inside a docker container that can be used to build and run the react app.

`./wasm-work.sh` dumps you into a shell inside a docker container that can be used to build the rust web assembly package.
