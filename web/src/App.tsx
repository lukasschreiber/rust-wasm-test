import { useEffect, useRef, useState } from "react"
import * as wasm from "wasm-example"

function App() {
  const [showCanvas, setShowCanvas] = useState(false)

  return (
    <div className="App">
      <button onClick={() => { setShowCanvas(true); wasm.instanced_test() }}>Show Rendering</button>
      <canvas id="canvas2" style={{ display: (showCanvas ? "block" : "none"), width: "100%", height: "50%" }}></canvas>

      <canvas id="canvas1" style={{ display: (showCanvas ? "block" : "none"), width: "100%", height: "50%" }}></canvas>

    </div>
  )
}

export default App
