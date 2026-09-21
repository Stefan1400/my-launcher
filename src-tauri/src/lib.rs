use tauri::Manager;

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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_youtube])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}