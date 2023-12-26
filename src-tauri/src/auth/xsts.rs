use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Response {
    display_claims: DisplayClaims,
}

#[derive(Debug, Deserialize)]
struct DisplayClaims {
    xui: Vec<Xui>,
}

#[derive(Debug, Deserialize)]
struct Xui {
    uhs: String,
}

pub async fn xsts(xbltoken: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
    let body = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [xbltoken],
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }
    });

    let response = client.post(url).json(&body).send().await?;

    let response_body: Response = response.json().await?;

    let xsts_token = response_body.display_claims.xui[0].uhs.clone();

    Ok(xsts_token)
}
