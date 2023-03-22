import { useEffect, useRef, useState } from "react"
import * as wasm from "wasm-example"

function App() {
  const [showCanvas, setShowCanvas] = useState(false)

  return (
    <div className="App">
      <button onClick={() => { setShowCanvas(true); wasm.instanced_test() }}>Show Rendering</button>
      <canvas style={{ display: (showCanvas ? "block" : "none"), width: "100%", height: "100%" }}></canvas>
    </div>
  )
}

export default App
