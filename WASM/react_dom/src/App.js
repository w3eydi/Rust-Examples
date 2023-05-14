import logo from './logo.svg';
import './App.css';
import init, { add_and_show } from "rust_wasm";
import { useEffect } from 'react';

function App() {
  useEffect(() => {
    init().then(() => {
      add_and_show(1,2);
    });
  }, []);

  return (
    <div className="App">
    </div>
  );
}

export default App;
