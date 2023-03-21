import { useState } from "react"
import * as wasm from "wasm-example"

function App() {
  const [showCanvas, setShowCanvas] = useState(false)

  return (
    <div className="App">
      {!showCanvas && <button onClick={() => { setShowCanvas(true); wasm.test3_d() }}>Show Rendering</button>}
      <canvas style={{ display: (showCanvas ? "block" : "none"), width: "100%", height: "100%" }}></canvas>
    </div>
  )
}

export default App
