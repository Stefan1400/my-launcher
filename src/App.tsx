import { invoke } from "@tauri-apps/api/core";
import Button from "./components/button";

function App() {
  const openProgram = async (invocation: string) => {
    try {
      await invoke(invocation);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <main className="launcher">
      <div className="launcher-container">
        <section className="launcher-section">
          <div className="section-header">
            <h2>Work / Study</h2>
          </div>

          <div className="button-grid">
            <Button buttonName="Preply" openProgram={openProgram} invocation="open_preply" />
            <Button buttonName="Ato" openProgram={openProgram} invocation="open_ato" />
            <Button buttonName="ChatGPT" openProgram={openProgram} invocation="open_chat_gpt" />
          </div>
        </section>

        <section className="launcher-section">
          <div className="section-header">
            <h2>Entertainment</h2>
          </div>

          <div className="button-grid">
            <Button buttonName="YouTube" openProgram={openProgram} invocation="open_youtube" />
            <Button buttonName="Spotify" openProgram={openProgram} invocation="open_spotify" />
            <Button buttonName="Tubi" openProgram={openProgram} invocation="open_tubi" />
          </div>
        </section>

        <section className="launcher-section">
          <div className="section-header">
            <h2>Gaming</h2>
          </div>

          <div className="button-grid">
            <Button buttonName="Roblox" openProgram={openProgram} invocation="open_roblox" />
            <Button buttonName="StardewValley" openProgram={openProgram} invocation="open_stardew_valley" />
            <Button buttonName="Fortnite" openProgram={openProgram} invocation="open_fortnite" />
          </div>
        </section>
      </div>
    </main>
  );
}

export default App;