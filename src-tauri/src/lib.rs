use tauri::{Manager, WindowEvent};

#[tauri::command]
fn open_youtube() -> Result<(), String> {
    std::process::Command::new(
        r"C:\Users\Jari\AppData\Local\Microsoft\WindowsApps\DuckDuckGo.exe",
    )
    .arg("https://www.youtube.com")
    .spawn()
    .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_spotify() -> Result<(), String> {
    std::process::Command::new("explorer.exe")
        .arg(r"shell:AppsFolder\SpotifyAB.SpotifyMusic_zpdnekdrzrea0!Spotify")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_preply() -> Result<(), String> {
    std::process::Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
        .arg("https://preply.com/en/home")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_roblox() -> Result<(), String> {
    std::process::Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
        .arg("https://www.roblox.com/home")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_ato() -> Result<(), String> {
    std::process::Command::new(
        r"C:\Users\Jari\AppData\Local\Programs\Microsoft VS Code\Code.exe"
    )
    .arg(r"C:\Users\Jari\OneDrive\ato")
    .spawn()
    .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_chat_gpt() -> Result<(), String> {
    std::process::Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
        .arg("https://chatgpt.com/")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_stardew_valley() -> Result<(), String> {
    std::process::Command::new("explorer.exe")
        .arg("steam://rungameid/413150")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_tubi() -> Result<(), String> {
    std::process::Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
        .arg("https://tubitv.com/")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_fortnite() -> Result<(), String> {
    std::process::Command::new("explorer.exe")
        .arg("com.epicgames.launcher://apps/fn%3A4fe75bbc5a674f4f9b356b5c90567da5%3AFortnite?action=launch&silent=true")
        .spawn()
        .map_err(|error| error.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    use tauri_plugin_global_shortcut::ShortcutState;

                    if event.state() == ShortcutState::Pressed {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(),
        )
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code,
                    GlobalShortcutExt,
                    Modifiers,
                    Shortcut,
                };

                let shortcut = Shortcut::new(
                    Some(Modifiers::ALT),
                    Code::Space,
                );

                app.global_shortcut().register(shortcut)?;
            }

            if let Some(window) = app.get_webview_window("main") {
                window.set_always_on_top(true)?;

                window.on_window_event({
                    let window = window.clone();

                    move |event| {
                        if let WindowEvent::CloseRequested { api, .. } = event {
                            api.prevent_close();
                            let _ = window.hide();
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_youtube,
            open_spotify,
            open_preply,
            open_roblox,
            open_ato,
            open_chat_gpt,
            open_stardew_valley,
            open_tubi,
            open_fortnite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}