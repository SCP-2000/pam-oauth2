//! API clients for exchanging access tokens for user information

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
}

impl GithubUser {
    pub fn get(token: &str) -> Result<Self> {
        Ok(ureq::get("https://api.github.com/user")
            .set("accept", "application/json")
            .set("authorization", format!("token {}", token).as_str())
            .call()?
            .into_json()?)
    }
}
