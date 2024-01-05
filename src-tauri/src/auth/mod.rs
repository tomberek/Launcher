mod bearer_token;
mod server;
mod xbox;
mod xsts;


use server::start_server;
use xbox::xbox;
use bearer_token::bearer_token;
use xsts::xsts;


pub async fn auth() -> Result<(String, String), Box<dyn std::error::Error>> {
    let (access_token, _state) = start_server()?;

    let (userhash, xboxtoken) = xbox(&access_token).await?;
    print!("Userhash: {} Token: {}", userhash, xboxtoken);
    let xsts = xsts(&userhash).await?;
    let (bearer_token, uuid) = bearer_token(&xsts, &userhash).await?;



    Ok((bearer_token, uuid))
}
