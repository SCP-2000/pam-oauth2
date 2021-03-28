pub mod ffi;
pub mod github;
use ffi::*;

#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
) -> PamReturnCode {
    let id = "2d8d2c5e9878098d0657";
    let allow = "NickCao";
    let code = match github::prepare(&id) {
        Ok(c) => c,
        _ => return PamReturnCode::SYSTEM_ERR,
    };
    if let Err(_) = pam_prompt(
        pamh,
        PamMessageStyle::TEXT_INFO,
        format!(
            "go to {} and input {}",
            code.verification_uri, code.user_code
        )
        .as_str(),
    ) {
        return PamReturnCode::SYSTEM_ERR;
    }
    let user = match github::resolve(&id, code) {
        Ok(u) => u,
        _ => return PamReturnCode::SYSTEM_ERR,
    };
    if user == allow {
        PamReturnCode::SUCCESS
    } else {
        PamReturnCode::PERM_DENIED
    }
}
