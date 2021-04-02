//!A simplified implementation of rfc8628, OAuth 2.0 Device Authorization Grant
//!```rust
//!let req = DeviceAuthorizationRequest {
//!  client_id: "2d8d2c5e9878098d0657",
//!  scope: None,
//!};
//!let resp: DeviceAuthorizationResponse = req
//!    .roundtrip("https://github.com/login/device/code")
//!    .unwrap();
//!println!("please go to {} and enter code {}", resp.verification_uri, resp.user_code);
//!// wait for user
//!let req = DeviceAccessTokenRequest::new(&req, &resp);
//!let resp: DeviceAccessTokenResponse = req
//!    .roundtrip("https://github.com/login/oauth/access_token")
//!    .unwrap();
//!println!("access token is {}", resp.access_token);

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DeviceAuthorizationRequest<'a, 'b> {
    pub client_id: &'a str,
    pub scope: Option<&'b str>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceAuthorizationResponse {
    device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct DeviceAccessTokenRequest<'a, 'b> {
    grant_type: &'static str,
    device_code: &'a str,
    client_id: &'b str,
}

#[derive(Deserialize, Debug)]
pub struct DeviceAccessTokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub error: Option<String>,
}

impl<'a, 'b> DeviceAccessTokenRequest<'a, 'b> {
    pub fn new(
        auth: &'b DeviceAuthorizationRequest,
        resp: &'a DeviceAuthorizationResponse,
    ) -> Self {
        DeviceAccessTokenRequest {
            grant_type: "urn:ietf:params:oauth:grant-type:device_code",
            device_code: &resp.device_code,
            client_id: auth.client_id,
        }
    }
}

pub trait Roundtrip<T>
where
    Self: Serialize,
    T: for<'de> Deserialize<'de>,
{
    fn roundtrip(&self, endpoint: &str) -> Result<T> {
        Ok(ureq::post(endpoint)
            .set("accept", "application/json")
            .send_string(&serde_urlencoded::to_string(self)?)?
            .into_json()?)
    }
}

impl Roundtrip<DeviceAuthorizationResponse> for DeviceAuthorizationRequest<'_, '_> {}
impl Roundtrip<DeviceAccessTokenResponse> for DeviceAccessTokenRequest<'_, '_> {}
