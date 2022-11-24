# The Problems

I want to produce spectrographs of audio files completely in the browser and I want it to be fast.

**What do I want to do?**

1. Use rust compiled to web assembly for the FFT.
2. Use pixi.js to provide webGL access for visualization.
3. Use TypeScript React for the wrapping application.
4. Use docker to run a local development version.
5. Use serverless and terraform to run a publicly accessible version.

**Why do I want to do that?**

1. I want to play with rust because I'm skeptical.
2. pixi.js seems neat from the little I've played with it.
3. I need to maintain familiarity with Typescript React.
4. Easy to run and demo without having to worry about host configuration management.
5. This part is boring so I'll use what I know.

There are a lot of individual technical problems to solve to make this work.

---

## Problem 1: Performing the FFT on a wav file.

```
Problem: How do you perform a FFT on either mono and stereo wav files in Rust?

Approach: Build experiment to load the file, process it, and output to something that lets me analyze success.
```

### Experiment

Built `fft/fftplay` and it does the following:

1. Loads a mono or stereo wave file.
2. Flattens it to mono if it's stereo.
3. Calculates the FFT frame size based on the desired resolution.
4. Pads the sample buffer to match.
5. Processes the data in place.
6. Simultaneously calculates frequency, magnitude, and phase while writing the output to CSV.

A python plotter is included to visualize the results in the CSV.

### Run It

To build and run the fft.

1. Running `./work.sh` in `fft/` builds and launches a docker container that can build the rust project.
2. Once the container launches, `cd fftplay`
3. `cargo build`
4. `target/debug/fftplay sample1.wav 2` will produce the output CSV.

To display the csv data.

1. Install python3 and virtualenv.
2. Create a virtualenv, activate it, and install the `requirements.txt`.
3. `python plot.py`

---

## Problem 2: WebAssembly I/O Paths

```
Problem: How do you get samples into web assembly and visualizabel data out?

Approach: Build a barebones web assembly/javascript project to inject "right shaped" data into web assembly and then render it to text in the javascript.

```

Getting data between web assembly and the javascripts heap [is not straight forward](https://rustwasm.github.io/book/game-of-life/implementing.html#interfacing-rust-and-javascript).

My initial research tells me a few things:

1. It's complicated, so it needs to be wrapped in a little sugar to make it easier to understand for observers.
2. It's best to not move data back and forth between the java heap and the web assembly memory so the source of truth for rendering should stay where it is if possible.
3. Serialization is expensive too, so try to avoid it unless the readability (maintainability) is a bigger benefit than the performance hit.

### Experiment

Built `seam/` and it does the following:

1. Provides a rust built wasm package that can take in samples (i16) and return outputs (f32).
2. It ingests this data in two different ways and provides performance data.
3. It outputs this data in three different ways and provides performance data.

The conclusion is that pushing data into the web assembly via a `&js_sys::Int16Array` reference is the fastest.

Rendering the data from inside the wasm was fastest, but that may be difficult to do. Problem 3 will discover that. If it is, the next fastest option is to access the wasm memory directly.

### Run It

To build the wasm library:

1. Install docker.
2. Run `./build.sh`.

To run the experiment.

1. Install python3.
2. Run `./run.sh`
3. Open the browser and go to http://localhost:8000/
4. Note times in the console log.
5. Open `index.html` and change line 17 to false.
6. Refresh the browser and note the times in the console log.

### Notes

https://rustwasm.github.io/wasm-bindgen/api/js_sys/index.html

https://rustwasm.github.io/docs/book/game-of-life/implementing.html

https://rustwasm.github.io/docs/wasm-bindgen/

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

https://github.com/rustwasm/wasm-bindgen/issues/2200#issuecomment-1169583390

---

## Problem 3: Data Display

`How do you display the data?`

The data has be to converted into something that pixi.js can render. This is achievable in a few ways.

1. Directly render shapes from the data.
2. Generate an SVG and inject it into the pixi.js loader.

Direct render doesn't require image encoding.

SVG will be easier to load and storeable for later.

Also, the question remains whether or not pixi.js functions can be handed into the wasm.

### Experiment

Built `display/` and it does the following:

1. Open's PIXI app and does some basic stuff.
2. Exports some javascript functions to the wasm to understand how that works.
3. Calls the exported functions as a quick test.

As I worked through it, I realized how difficult it was going to be to create and manage Graphics objects in the wasm. It would probably make culling faster, but there was a lot
of module passing and bufoonery happening to get it so that I could create a Graphics object from a handed in javascript function.

I decided it wasn't worth the trouble, but if I have performance problems later, it's a place to look.

Update: Graphics object generation is fast enough for real time feeds. No way that adding a step of svg generation and loading will be faster.

### Run It

To build the wasm library:

1. Install docker.
2. Run `./build.sh`.

To run the experiment.

1. Install python3.
2. Run `./run.sh`
3. Open the browser and go to http://localhost:8000/

---

## Stretch 1: Support Overlapping

`Implement FFT overlapping to get better frequency representation of our time-domain signal.`

### Resources

https://holometer.fnal.gov/GH_FFT.pdf

## Stretch 2: Support MP3 and/or FLAC

`How do you convert and then provide the wav data?`

## Other Notes

https://pages.mtu.edu/~suits/notefreqs.html
