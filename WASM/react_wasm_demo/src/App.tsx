import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import init, { add } from "wasm_lib";

function App() {
  const [answer, setAnswer] = useState(0);
  useEffect(() => {
    init().then(() => {
      setAnswer(add(1, 1));
    })
  }, [])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          add() function From WASM: 1 + 1 = <code>{answer}</code>
        </p>
      </header>
    </div>
  );
}

export default App;
