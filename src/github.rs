use anyhow::anyhow;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct CodeRequest<'a> {
    client_id: &'a str,
}

#[derive(Deserialize)]
pub struct CodeResponse {
    device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    interval: u64,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    client_id: &'a str,
    device_code: &'a str,
    grant_type: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    interval: Option<u64>,
}

#[derive(Deserialize)]
struct UserInfo {
    login: String,
}

pub fn prepare(client_id: &str) -> Result<CodeResponse> {
    let code: CodeResponse = ureq::post("https://github.com/login/device/code")
        .set("Accept", "application/json")
        .send_json(ureq::json!(CodeRequest { client_id }))?
        .into_json()?;
    Ok(code)
}

pub fn resolve(client_id: &str, code: CodeResponse) -> Result<String> {
    let mut interval = code.interval;
    let token = loop {
        let token: TokenResponse = ureq::post("https://github.com/login/oauth/access_token")
            .set("Accept", "application/json")
            .send_json(ureq::json!(TokenRequest {
                client_id: client_id,
                device_code: &code.device_code,
                grant_type: "urn:ietf:params:oauth:grant-type:device_code"
            }))?
            .into_json()?;
        match token {
            TokenResponse {
                access_token: Some(t),
                ..
            } => break Ok(t.to_owned()),
            TokenResponse { error: Some(e), .. } if e == "authorization_pending" => (),
            TokenResponse {
                error: Some(e),
                interval: Some(i),
                ..
            } if e == "slow_down" => interval = i,
            _ => break Err(anyhow!("authorization failed")),
        }
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }?;
    let user: UserInfo = ureq::get("https://api.github.com/user")
        .set("Accept", "application/json")
        .set("Authorization", format!("token {}", token).as_str())
        .call()?
        .into_json()?;
    Ok(user.login)
}
