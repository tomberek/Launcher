pub mod server;
pub mod xbox;

use xbox::xbox;

pub async fn auth() -> Result<(), Box<dyn std::error::Error>> {
    let (access_token, state) = server::start_server()?;

    let (userhash, xuid) = xbox(&access_token).await?;
    print!("Userhash: {} Xuid: {}", userhash, xuid);
    Ok(())
}
