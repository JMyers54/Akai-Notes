import "./App.css";
import { Router, Route} from '@solidjs/router';

function App() {
  return (
    <Router>
        <Route path = "/" component={() => (
        <main class="container">
          <h1>Akai Notes</h1>
          <button class="open-proyect">Abrir Proyecto</button>
          <button class="create-proyect">Crear Proyecto</button>
        </main>
        )} />

    </Router>
  );
}

export default App;

