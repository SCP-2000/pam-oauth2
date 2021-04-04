pub mod api;
pub mod ffi;
pub mod oauth2;
use crate::api::*;
use crate::ffi::*;
use crate::oauth2::*;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

///An example pam module for authenticating against github
///
///#### pam config
///`auth required <path to module> <path to config>`
///#### config
///```yaml
///client_id: <github oauth2 client id>
///user_mapping:
///  foo: [ foo Foo ]
///  bar: [ boo boop ]
///```
#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
) -> PamReturnCode {
    let res = authenticate(pamh, flag, argc, argv);
    if res.is_ok() {
        PamReturnCode::SUCCESS
    } else {
        PamReturnCode::PERM_DENIED
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    client_id: String,
    user_mapping: HashMap<String, HashSet<String>>,
}

fn authenticate(
    pamh: &PamHandle,
    _flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
) -> Result<()> {
    let pam_user = pam_get_user(pamh, "").map_err(|_| anyhow!(""))?;
    let args = pam_get_args(argc, argv).map_err(|_| anyhow!(""))?;
    if args.len() != 1 {
        return Err(anyhow!("wrong number of arguments"));
    }
    let config = std::fs::File::open(&args[0])?;
    let config: Config = serde_yaml::from_reader(config)?;
    let req = oauth2::DeviceAuthorizationRequest {
        client_id: &config.client_id,
        scope: None,
    };
    let resp: oauth2::DeviceAuthorizationResponse =
        req.roundtrip("https://github.com/login/device/code")?;
    ffi::pam_prompt(
        pamh,
        PamMessageStyle::PROMPT_ECHO_OFF,
        &format!(
            "please go to {} and input {} then press enter here\n",
            resp.verification_uri, resp.user_code
        ),
    )
    .map_err(|_| anyhow!(""))?;
    let req = oauth2::DeviceAccessTokenRequest::new(&req, &resp);
    let resp: oauth2::DeviceAccessTokenResponse =
        req.roundtrip("https://github.com/login/oauth/access_token")?;
    let token = resp.access_token.ok_or_else(|| anyhow!(""))?;
    let user = GithubUser::get(&token)?;
    let set = config
        .user_mapping
        .get(&pam_user)
        .ok_or_else(|| anyhow!(""))?;
    if set.contains(&user.login) {
        Ok(())
    } else {
        Err(anyhow!(""))
    }
}
