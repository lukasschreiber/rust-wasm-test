import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import './index.css'
import * as wasm from "wasm-example"

wasm.greet()
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
