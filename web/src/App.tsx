import { useContext, useEffect } from "react"
import { RenderingContext } from "./context"

function App() {
  const rendering = useContext(RenderingContext)

  useEffect(() => {
    console.log("useEffect")
    rendering.create_window("canvas1")
    rendering.create_window("canvas2")
  })

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <canvas id="canvas2" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
    </div>
  )
}

export default App
