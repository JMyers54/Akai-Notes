import { createSignal, onMount, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import type { CurrentProject } from "./types";

function Editor() {
    const [path, setPath] = createSignal<string | null>(null);

    onMount(async () => {
        const project = await invoke<CurrentProject | null>(
            "get_current_project");
        setPath(project?.path ?? null);
    });
    return (
        <div class="Workspace">
            <aside class="sidebar">
                <h2>Akai Notes</h2>
                <Show when={path()}>
                    <p class="project-path" title={path()!}>{path()}</p>
                </Show>
                <p class="OpenHolder">Explorador: T-011</p>
            </aside>
            <main class="editor">
                <textarea placeholder="ya puedes escribir"></textarea>
            </main>
        </div>
    );
}

export default Editor;