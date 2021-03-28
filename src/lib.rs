pub mod ffi;
use ffi::*;
use oauth2::basic::BasicClient;
use oauth2::devicecode::StandardDeviceAuthorizationResponse;
use oauth2::ureq::http_client;
use oauth2::*;

#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
) -> PamReturnCode {
    let args = pam_get_args(argc, argv).unwrap();
    let user = pam_get_user(pamh, "").unwrap();
    pam_prompt(
        pamh,
        PamMessageStyle::TEXT_INFO,
        format!("args: {:?}, user: {}", args, user).as_str(),
    )
    .unwrap();
    let client_id = ClientId::new("2d8d2c5e9878098d0657".to_string());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap();
    let token_url =
        TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap();
    let device_auth_url =
        DeviceAuthorizationUrl::new("https://github.com/login/device/code".to_string()).unwrap();
    let client = BasicClient::new(client_id, None, auth_url, Some(token_url))
        .set_device_authorization_url(device_auth_url)
        .set_auth_type(AuthType::RequestBody);
    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()
        .unwrap()
        .request(http_client)
        .unwrap();
    pam_prompt(
        pamh,
        PamMessageStyle::TEXT_INFO,
        format!(
            "url: {}, code: {}",
            details.verification_uri().to_string(),
            details.user_code().secret().to_string()
        )
        .as_str(),
    )
    .unwrap();
    let token = client
        .exchange_device_access_token(&details)
        .request(
            http_client,
            std::thread::sleep,
            Some(std::time::Duration::from_secs(60)),
        )
        .unwrap();
    PamReturnCode::SUCCESS
}
