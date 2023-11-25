#[cfg(desktop)]

mod auth;

use auth::auth as authRS;



#[tauri::command]
async fn auth() -> Result<(), String> {
    authRS().await.map_err(|e| e.to_string())
}



mod desktop;

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!(auth))
        .expect("error while running tauri application");
}


