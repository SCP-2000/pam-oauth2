pub mod error;

use crate::error::PamError;
use pam_sys::types::*;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

#[no_mangle]
#[allow(unused_variables, improper_ctypes_definitions)]
pub extern "C" fn pam_sm_authenticate(
    pamh: &PamHandle,
    flag: PamFlag,
    argc: c_int,
    argv: *const *const c_char,
    ) -> PamReturnCode {
    let args = extract_args(argc, argv);
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

fn string_from_ptr(ptr: *const c_char) -> Result<String, std::ffi::IntoStringError> {
    let s = unsafe {CStr::from_ptr(ptr)};
    s.to_owned().into_string()
}

pub fn extract_args(
    argc: c_int,
    argv: *const *const c_char,
    ) -> Result<Vec<String>, std::ffi::IntoStringError> {
    (0..argc)
        .map(|i| string_from_ptr(unsafe {*argv.offset(i as isize) }) )
        .collect()
}

pub fn pam_get_user(pamh: &PamHandle, prompt: &str) -> Result<String, Box<dyn Error>> {
    let mut user: *const c_char = std::ptr::null();
    let prompt = CString::new(prompt)?;
    match pam_sys::wrapped::get_user(pamh, &mut user, prompt.as_ptr()) {
        PamReturnCode::SUCCESS => Ok(string_from_ptr(user)?),
        e => Err(Box::new(PamError::new(e))),
    }
}

pub fn pam_get_item<T>(
    pamh: &PamHandle,
    item_type: PamItemType,
    ) -> Result<Option<&T>, Box<dyn Error>> {
    let mut ptr: *const c_void = std::ptr::null();
    match pam_sys::wrapped::get_item(pamh, item_type, &mut ptr) {
        PamReturnCode::SUCCESS => Ok(unsafe { ptr.cast::<T>().as_ref() }),
        e => Err(Box::new(PamError::new(e))),
    }
}

pub fn pam_get_string(pamh: &PamHandle, itype: PamItemType) -> Result<String, Box<dyn Error>> {
    let s = pam_get_item(pamh, itype)?;
    let s = s.ok_or(PamError::new(PamReturnCode::SYSTEM_ERR))?;
    Ok(string_from_ptr(s)?)
}

pub fn pam_prompt(
    pamh: &PamHandle,
    style: PamMessageStyle,
    prompt: &str,
    ) -> Result<String, Box<dyn Error>> {
    let conv: &PamConversation = pam_get_item(pamh, PamItemType::CONV)?.ok_or(PamError::new(PamReturnCode::SYSTEM_ERR))?;
    let callback = conv.conv.ok_or(PamError::new(PamReturnCode::SYSTEM_ERR))?;
    let prompt = CString::new(prompt)?;
    let mut msg: *mut PamMessage = &mut PamMessage {
        msg: prompt.as_ptr(),
        msg_style: style as i32,
    };
    let mut resp: *mut PamResponse = std::ptr::null_mut();
    match PamReturnCode::from(callback(1, &mut msg, &mut resp, conv.data_ptr)) {
        PamReturnCode::SUCCESS => (),
        e => return Err(Box::new(PamError::new(e))),
    }
    unsafe { resp.as_ref().ok_or("resp err")?.resp.as_ref()}.map_or(Ok("".to_string()), |r| Ok(string_from_ptr(r)?))
}
