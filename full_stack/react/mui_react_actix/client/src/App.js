import './App.css';
import { Button, Stack, Rating } from '@mui/material/';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <Button variant="contained">Hello World</Button>

        <Stack spacing={1}>
          <Rating name="half-rating" defaultValue={2.5} precision={0.5} />
        </Stack>
        <h1 className="text-3xl underline">
          Hello world!
        </h1>
      </header>

    </div>
  );
}

export default App;
