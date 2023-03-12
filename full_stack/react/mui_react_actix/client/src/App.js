import Navi from './components/navi/Navi';
import { Container } from '@mui/material';
import Dashboard from './components/root/Dashboard';

function App() {
  return (
    <Container maxWidth="xl">
      <Navi />
      <Dashboard/>
    </Container>
  );
}

export default App;
