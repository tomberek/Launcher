use minecraft_essentials::{Oauth, CustomAuthData};
use serde_json::Value;
use tauri::App;
use tauri_plugin_http::{reqwest, Error};

#[tauri::command]
async fn auth() -> Result<CustomAuthData, String> {
    handle_auth().await.map_err(|e| e.to_string())
}

async fn handle_auth() -> Result<CustomAuthData, Box<dyn std::error::Error>> {
    let auth = Oauth::new("6a6bf548-5a82-41f5-9451-88b334cdc77f", None);
    let window_url = auth.url();

    let _ = open::that(window_url);

    let auth_info = auth.launch(false, "bAX8Q~biVLXbokLtT6ddhz_e8xm1WALle43XmbLh").await?;

    Ok(auth_info)
}
#[tauri::command]
async fn launch(version: String) -> Result<(), Error> {
    let clientversion = version.clone(); // This is the version you want to download
    let url = format!("https://github.com/TeaClientMC/Client/releases/download/{}/{}.jar", clientversion, version.clone());

    // Create a GET request to the specified URL
    let response = reqwest::get(&url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Get the bytes of the .jar file
        let jar_bytes = response.bytes().await?;

        // Define the path where you want to save the .jar file
        let jar_path = format!("~/.teaclient/jre/{}.jar", version.clone());

        // Write the bytes to a fileErr(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download Jar"))
        
        std::fs::write(jar_path.clone(), jar_bytes)?;

        println!("Downloaded and saved the .jar file to: {}", jar_path.clone());
        Ok(()) // Return Ok(()) to indicate success
    } else {
        Err(tauri_plugin_http::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Failed to download Jar")))
    }
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

    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup = Some(Box::new(setup));
        self
    }

    pub fn run(self) {
        let port = 8000;
        let setup = self.setup;
        tauri::Builder::default()
            .plugin(tauri_plugin_clipboard_manager::init())
            .plugin(tauri_plugin_localhost::Builder::new(port).build())
            .plugin(tauri_plugin_http::init())
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