import React, { useEffect, useState } from "react";

import { Application, Graphics } from "pixi.js";
import { WaveFile } from "wavefile";

import init, { FFTAnalysis } from "wasm-fft";
//import memory from "wasm-fft/wasm_fft_bg.wasm";     // None of this works
//import { memory } from "wasm-fft/wasm_fft_bg.wasm"; //      '''

import { HostedWavFile, WavSpec } from "./models";

const resolutionHz = 2;
const maxFreq = 8000; // B8 note is 7902 Hz

export default function AnalysisComponent(props: {
  hostedWavFile: HostedWavFile;
  pixiApp: Application;
}) {
  const [loadingWavFile, setLoadingWavFile] = useState(true);
  const [wavFile, setWavFile] = useState<WaveFile | null>(null);
  const [wavSpec, setWavSpec] = useState<WavSpec | null>(null);
  const [visualization, setVizulaization] = useState<Graphics | null>(null);
  const [loadStatus, setLoadStatus] = useState("Loading...");
  const [fftStatus, setFFTStatus] = useState("Preparing...");
  const [analysis, setAnalysis] = useState<FFTAnalysis | null>(null);
  const [mangitudes, setMagnitudes] = useState<Float32Array | null>(null);

  const [wavLoaded, setWavLoaded] = useState(false);

  const [fftComplete, setFFTComplete] = useState(false);

  const [renderComplete, setRenderComplete] = useState(false);

  let runFFT = () => {
    if (wavFile && wavSpec && !analysis) {
      const run = async () => {
        const wasmModule = await init();

        let samples = Int16Array.from(wavFile!.getSamples(true));

        const fft = FFTAnalysis.new(
          samples,
          wavSpec!.sampleRate,
          wavSpec!.channels,
          resolutionHz
        );

        setAnalysis(fft);

        setMagnitudes(
          new Float32Array(
            wasmModule.memory.buffer,
            fft.get_scaled_magnitude_ptr(),
            fft.length()
          )
        );
      };

      setFFTStatus("Running...");
      const start = performance.now();
      run().then(() => {
        const end = performance.now();
        setFFTStatus("Complete (" + Math.round(end - start) + "ms)");
      });
    } else if (!wavFile) {
      throw new Error("WavFile is null");
    } else if (!wavSpec) {
      throw new Error("WavSpec is null");
    }
  };

  let renderFFT = () => {
    if (analysis) {
      if (visualization) {
        props.pixiApp.stage.removeChildren();
        props.pixiApp.stage.addChild(visualization);
      } else {
        const totalFrames = analysis.length() / analysis.frame_size();
        //const frameLength = analysis.frame_size() / 2; // Second half is mirror of first.
        const frameLength = maxFreq / resolutionHz; // Limit to 8KHz (Musical Range)

        // Iterate the frames of the analysis.
        const graphics = new Graphics();
        for (let frameNumber = 0; frameNumber < totalFrames; frameNumber++) {
          const frameStartIndex = frameNumber * analysis.frame_size();
          // Iterate the magnitudes of the frame.
          for (
            let relativeIndex = 0;
            relativeIndex < frameLength;
            relativeIndex++
          ) {
            const W = 2;
            // Render each magnitude as a rectangle.
            const index = frameStartIndex + relativeIndex;
            //const shade = (Math.floor(mangitudes![index] * 240) + 15) * 0x10000;
            const shade = Math.floor(mangitudes![index] * 0xffffff);
            graphics.beginFill(shade);
            graphics.drawRect(frameNumber * W, relativeIndex, W, 1);

            props.pixiApp.stage.removeChildren();
            props.pixiApp.stage.addChild(graphics);
          }
        }
        graphics.endFill();
        graphics.width = 800;
        graphics.height = 800;
        setVizulaization(graphics);
      }
    } else {
      throw new Error("Analysis is null");
    }
  };

  useEffect(() => {
    if (!wavFile && !wavSpec) {
      // Load the wav file.
      fetch(props.hostedWavFile.url).then((data) => {
        data.arrayBuffer().then((buffer) => {
          const wf = new WaveFile();
          wf.fromBuffer(new Uint8Array(buffer));
          setWavFile(wf);
          //console.log(wf.fmt);                         // This is fine
          //let specs = wf.fmt;                          // This is fine, too.
          //let sampleRate = specs.sampleRate;                // This throws TS2339.
          //console.log(wf.fmt.sampleRate);              // This prints value to console, but throws TS2339. 🤷🏻‍♂️
          let spec = JSON.parse(JSON.stringify(wf.fmt)); // Hacky workaround.
          setWavSpec({
            sampleRate: spec.sampleRate,
            channels: spec.numChannels,
            bitsPerSample: spec.bitsPerSample,
          });
          console.log(wf);
          setLoadStatus("Loaded " + buffer.byteLength + " samples");
          setLoadingWavFile(false);
          setFFTStatus("Ready");
          setWavLoaded(true);
        });
      });
    }
  }, [props.hostedWavFile]);

  const wavFileLoaded = wavSpec !== null && wavFile !== null;

  return (
    <>
      <div
        style={{
          background: "#f0f0f0",
          border: "1px solid black",
          padding: "10px",
          margin: "10px",
          borderRadius: "5px",
        }}
      >
        <p className="title">{props.hostedWavFile.name}</p>
        <p>
          <span className="label">Load:</span> {loadStatus}
        </p>
        <p>
          <span className="label">FFT:</span> {fftStatus}
        </p>
        <button onClick={runFFT} disabled={loadingWavFile || analysis !== null}>
          Run
        </button>
        <button onClick={renderFFT} disabled={analysis === null}>
          View
        </button>
      </div>
    </>
  );
}
