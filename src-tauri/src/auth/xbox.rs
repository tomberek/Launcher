use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use reqwest::Error;
use serde_json::json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResponseBody {
   token: String,
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


pub async fn xbox(accesstoken: &str) -> Result<(String, String), Error> {
 let client = reqwest::Client::new();

 let mut headers = HeaderMap::new();
 headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
 headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

 let body = json!({
 "Properties": {
     "AuthMethod": "RPS",
     "SiteName": "user.auth.xboxlive.com",
     "RpsTicket": format!("d={}", accesstoken) // your access token from the previous step here
 },
 "RelyingParty": "http://auth.xboxlive.com",
 "TokenType": "JWT"
 });

 let response = client.post("https://user.auth.xboxlive.com/user/authenticate")
 .headers(headers)
 .json(&body)
 .send()
 .await?;

 println!("Status: {}", response.status());
 println!("Headers:\n{:#?}", response.headers());

 let response_body: ResponseBody = response.json().await?;
 println!("Body:\n{:?}", response_body);

 // Extract the userhash and xboxtoken from the response body
 let userhash = response_body.display_claims.xui[0].uhs.clone();
 let xboxtoken = response_body.token.clone();

 // Drop the client to close the connection
 drop(client);

 Ok((userhash, xboxtoken))
}
