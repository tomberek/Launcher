use dotenv::dotenv;
use std::env;
use tauri::App;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

#[tauri::command]
async fn auth() {
    dotenv().ok();

    /*
     * All Credits go to Minecraft-Essentails.
     * Note: Minecraft-Essentails Is Licenced Under The Affero GPL 3.0
     */

    let client_id = "";
    let client_secret = env::var("Client_Secret").expect("Client_Secret Is Expected");
    let ouath = minecraft_essentials::Oauth::new(&client_id);
    print!("URL: {}", ouath.url());

    // let ouath_info = ouath.launch(false).await?;
}

#[tauri::command]
fn launch() {

    /*
     * All Credits go to Minecraft-Essentails.
     * Note: Minecraft-Essentails Is Licenced Under The Affero GPL 3.0
     */

    //TODO: Minecraft Launching Client.

    // minecraft_essentials::minecraft::new()
}

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .plugin(tauri_plugin_notification::init())
            .plugin(tauri_plugin_os::init())
            .plugin(tauri_plugin_http::init())
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_clipboard_manager::init())
            .plugin(tauri_plugin_process::init())
            .plugin(tauri_plugin_global_shortcut::Builder::new().build())
            .invoke_handler(tauri::generate_handler![auth, launch])
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
