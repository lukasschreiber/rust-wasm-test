import { createContext, useContext, useEffect, useRef, useState } from "react"
import * as wasm from "wasm-example"

const RenderingContext = createContext(new wasm.Rendering());

function App() {
  const rendering = useContext(RenderingContext)

  useEffect(() => {
    rendering.create_window()
  })

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <canvas id="canvas2" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
    </div>
  )
}

export default App
