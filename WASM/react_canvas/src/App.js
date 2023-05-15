import logo from './logo.svg';
import './App.css';
import init, { render } from "rust_wasm";
import { useEffect } from 'react';

function App() {
  useEffect(() => {
    init().then(() => {
      render("canvas");
    });
  }, []);
  return (
    <div className="App">
      <canvas id="canvas" width={400} height={400} />
    </div>
  );
}

export default App;
