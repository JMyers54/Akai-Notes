import "./App.css";
import { Router, Route} from '@solidjs/router';

function App() {
  return (
    <Router>
        <Route path = "/" component={() => (
        <main class="container">
          <h1>Akai Notes</h1>
        </main>
        )} />

    </Router>
  );
}

export default App;
