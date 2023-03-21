import { useEffect } from "react"
import * as wasm from "wasm-example"

function App() {
  return (
    <div className="App">
      <button onClick={() => wasm.test3_d()}>Test</button>
      <canvas></canvas>
      {wasm.funcy_string()}
    </div>
  )
}

export default App
