use pam_sys::types::{PamHandle, PamReturnCode};
use std::os::raw::{c_char, c_int};

#[no_mangle]
pub extern "C" fn pam_sm_authenticate(
    pamh: PamHandle,
    flag: c_int,
    argc: c_int,
    argv: *const *const c_char,
) -> PamReturnCode {
    return PamReturnCode::SUCCESS;
}
