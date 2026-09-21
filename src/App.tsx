import { invoke } from "@tauri-apps/api/core";

function App() {
  const openYouTube = async () => {
    try {
      await invoke("open_youtube");
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <main>
      <button onClick={openYouTube}>
        Open YouTube
      </button>
    </main>
  );
}

export default App;