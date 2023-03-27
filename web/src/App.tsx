import { useContext, useEffect } from "react"
import { RenderingContext } from "./context"

function App() {
  const rendering = useContext(RenderingContext)

  useEffect(() => {
    console.log("useEffect creation")
    let ref1 = rendering.create_window("canvas1");
    let ref2 = rendering.create_window("canvas2");
    console.log("useEffect creation done")

    return () => {
      console.log("useEffect cleanup")
      rendering.delete_window(ref1)
      rendering.delete_window(ref2)
      console.log("useEffect cleanup done")
    }
  })

  return (
    <div className="App">
      <canvas id="canvas1" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
      <canvas id="canvas2" style={{ display: "block", width: "100%", height: "50%" }}></canvas>
    </div>
  )
}

export default App
