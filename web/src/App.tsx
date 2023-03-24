import { useEffect, useRef, useState } from "react"
import * as wasm from "wasm-example"

function App() {
  useEffect(() => {
    wasm.instanced_test()
  })

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <canvas id="canvas2" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
    </div>
  )
}

export default App
