pub mod api;
pub mod ffi;
pub mod oauth2;
use crate::api::*;
use crate::ffi::*;
use crate::oauth2::*;

///An example pam module for authenticating against github
#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
) -> PamReturnCode {
    let req = oauth2::DeviceAuthorizationRequest {
        client_id: "2d8d2c5e9878098d0657",
        scope: None,
    };
    let resp: oauth2::DeviceAuthorizationResponse = req
        .roundtrip("https://github.com/login/device/code")
        .unwrap();
    println!("{:?}", resp);
    let req = oauth2::DeviceAccessTokenRequest::new(&req, &resp);
    ffi::pam_prompt(pamh, PamMessageStyle::PROMPT_ECHO_OFF, "press enter");
    let resp: oauth2::DeviceAccessTokenResponse = req
        .roundtrip("https://github.com/login/oauth/access_token")
        .unwrap();
    println!("{:?}", resp);
    if let oauth2::DeviceAccessTokenResponse {
        access_token: Some(access_token),
        ..
    } = resp
    {
        let resp = api::GithubUser::get(&access_token).unwrap();
        println!("{:?}", resp);
    }
    PamReturnCode::PERM_DENIED
}
