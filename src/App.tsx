import "./App.css";
import { createSignal, onMount, Show } from "solid-js";
import type { CurrentProject } from "./types";
import { Router, Route, useNavigate } from '@solidjs/router';
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import Editor from "./Editor";

function Home() {
  const navigate = useNavigate();
  const [checked, setChecked] = createSignal(false);

  onMount(async () => {
    try {
      const project = await invoke<CurrentProject | null>(
        "get_current_project");
      if (project) navigate("/editor", { replace: true });
    } catch {
      // Error::NotFound -> la carpeta ya no existe, mostrar la ho
    } setChecked(true);
  });

  async function openProject() {
    const dir = await open({ directory: true, multiple: false, title: "Abrir Proyecto" });
    if (!dir) return;
    await invoke("set_current_project", { path: dir });
    navigate("/editor");
  }

  async function createProject() {
    const dir = await open({
      directory: true, multiple: false,
      title: "Crear Proyecto"
    });
    if (dir) {
      await invoke("set_current_project", { path: dir });
      navigate("/editor");
    }
  }

  return (
    <Show when={checked()} fallback={<main
      class="container"><p>Cargando...</p></main>
    }>
      <main class="container">
        <h1>Akai Notes</h1>
        <button class="open-proyect" onClick={openProject}>Abrir Proyecto</button>
        <button class="create-proyect" onClick={createProject}>Crear Proyecto</button>
      </main>
    </Show>
  );
}

function App() {
  return (
    <Router>
      <Route path="/" component={Home} />
      <Route path="/editor" component={Editor} />
    </Router>
  );
}

export default App;

