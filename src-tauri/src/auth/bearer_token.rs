use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Response {
    username: String,
    access_token: String,
}

pub async fn bearer_token(userhash: &str, xsts_token: &str) -> Result<(String, String), Error> {
    let client = reqwest::Client::new();
    let url: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";

    let body = json!({
        "identityToken": format!("XBL3.0 x={};{}", userhash, xsts_token)
    });

    let response = client.post(url).json(&body).send().await?;

    let response_body: Response = response.json().await?;

    let uuid = response_body.username.clone();
    let access_token = response_body.access_token.clone();

    Ok((access_token, uuid))
}
