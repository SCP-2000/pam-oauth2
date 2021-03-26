pub mod ffi;
use ffi::*;

#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
) -> PamReturnCode {
    let args = pam_get_args(argc, argv);
    println!("{:?}", args);
    let user = pam_get_user(pamh, "who are you");
    println!("{:?}", user);
    let svc = pam_get_string(pamh, PamItemType::SERVICE);
    println!("{:?}", svc);
    let resp = pam_prompt(pamh, PamMessageStyle::TEXT_INFO, "hello pam");
    println!("{:?}", resp);
    let resp = pam_prompt(pamh, PamMessageStyle::PROMPT_ECHO_OFF, "enter something");
    println!("{:?}", resp);
    PamReturnCode::SUCCESS
}
