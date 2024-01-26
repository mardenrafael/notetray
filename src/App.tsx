import { invoke } from "@tauri-apps/api";
import { Plus } from "lucide-react";
import { useState } from "react";

function App() {
  const [noteName, setNoteName] = useState<string>("");

  function handleSubmit() {
    if (noteName === "") {
      return;
    }

    invoke("create_note", {
      notePayload: {
        name: noteName,
      },
    });
  }

  return (
    <div className="bg-slate-900 text-slate-100 h-screen flex flex-col select-none ">
      <header className="p-2 text-center">
        <h1 className="font-bold text-2xl">Note Tray</h1>
      </header>
      <main className="flex mt-4 justify-center items-center flex-col gap-4">
        <section>
          <div className="flex flex-1 flex-col items-center justify-center p-2">
            <label htmlFor="noteName">Nome da sua nova anotação</label>
            <input
              className="bg-slate-900 border border-slate-100 rounded text-slate-100 p-2 text-center focus:ring-0 focus:outline-none"
              type="text"
              placeholder="Nome"
              name="noteName"
              id="noteName"
              onChange={(e) => {
                const value = e.currentTarget.value;

                setNoteName(value);
              }}
            />
          </div>
        </section>

        <button
          className="cursor-default border border-slate-100 p-2 rounded hover:bg-slate-700 transition-all focus:ring-0 focus:outline-none"
          onClick={handleSubmit}
        >
          <Plus />
        </button>
      </main>
    </div>
  );
}

export default App;
