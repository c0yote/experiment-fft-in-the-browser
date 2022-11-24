import React, { useEffect, useState } from "react";

import { Application } from "pixi.js";

export default function PixiComponent(props: { pixiApp: Application }) {
  const canvasRef = React.createRef<HTMLDivElement>();

  useEffect(() => {
    if (canvasRef.current) {
      canvasRef.current.appendChild(props.pixiApp.view as any);
    }
  }, [canvasRef, props.pixiApp]);

  return (
    <>
      <div
        style={{
          border: "1px solid black",
          padding: "10px",
          margin: "10px",
          borderRadius: "5px",
          float: "left",
          backgroundColor: "black",
        }}
      >
        <div ref={canvasRef} />
      </div>
    </>
  );
}
