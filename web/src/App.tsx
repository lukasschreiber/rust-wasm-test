import { createContext, useContext, useEffect, useRef, useState } from "react"
import * as wasm from "wasm-example"

const RenderingContext = createContext((() => {
  const rendering = new wasm.RenderingNever();
  const proxy = rendering.get_proxy();
  try {
    rendering.run()
  } catch (error) {
    console.error(error)
  }
  return proxy
})());

function App() {
  const rendering = useContext(RenderingContext)

  useEffect(() => {
    rendering.send_event()
  })

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <canvas id="canvas2" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
    </div>
  )
}

export default App
