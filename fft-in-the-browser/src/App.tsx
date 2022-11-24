import React, { useEffect, useState } from "react";
import logo from "./logo.svg";

import { Application } from "pixi.js";

import "./App.css";
import { HostedWavFile } from "./models";
import PixiComponent from "./PixiComponent";
import AnalysisComponent from "./AnalysisComponent";

// #NoDBProblems amirite?
const hostedWavFiles: HostedWavFile[] = [
  {
    id: 0,
    name: "sis.wav",
    url: "http://localhost:3000/sis.wav",
    description: "Swiping in Space.",
  },
];

function App() {
  const [pixiApp, setPixiApp] = useState<Application>(
    new Application({ width: 800, height: 800 })
  );
  const [wavFiles, setWavFiles] = useState(hostedWavFiles);

  return (
    <div className="App">
      <div
        style={{
          boxShadow: "0px 2px 4px rgba(0, 0, 0, 0.12)",
          borderRadius: "5px",
          float: "left",
          //border: "1px solid #4b4b4b",
          padding: "8px",
          margin: "8px",
        }}
      >
        {wavFiles.map((wavFile) => {
          return (
            <AnalysisComponent
              key={wavFile["id"]}
              hostedWavFile={wavFile}
              pixiApp={pixiApp}
            />
          );
        })}
      </div>
      <div>
        <PixiComponent pixiApp={pixiApp} />
      </div>
    </div>
  );
}

export default App;
