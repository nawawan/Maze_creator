import { useState, useRef, useEffect } from 'react';
import './App.css';

import {draw_maze} from '../wasm/pkg/wasm';

function App() {
  const [count, setCount] = useState(0);
  
  const canvasRef = useRef<HTMLCanvasElement | null>(null);

  useEffect(() => {
    if(canvasRef.current){
      draw_maze();
    }
  }, []);


  return (
    <>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <canvas id="canvas" height="300" width="400" ref={canvasRef}></canvas>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
