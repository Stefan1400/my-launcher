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
    <main>
      <Button 
        buttonName='Youtube' 
        openProgram={openProgram} 
        invocation='open_youtube' 
      />

      <Button 
        buttonName='Spotify' 
        openProgram={openProgram} 
        invocation='open_spotify' 
      />

      <Button 
        buttonName='Preply' 
        openProgram={openProgram} 
        invocation='open_preply' 
      />

      <Button 
        buttonName='Roblox' 
        openProgram={openProgram} 
        invocation='open_roblox' 
      />

      <Button 
        buttonName='Ato' 
        openProgram={openProgram} 
        invocation='open_ato' 
      />

      <Button 
        buttonName='ChatGPT' 
        openProgram={openProgram} 
        invocation='open_chat_gpt' 
      />

    </main>
  );
}

export default App;