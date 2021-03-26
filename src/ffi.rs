pub use pam_sys::types::{PamFlag, PamHandle, PamItemType, PamMessageStyle, PamReturnCode};
pub use std::os::raw::{c_char, c_int};

use pam_sys::types::PamReturnCode::*;
use pam_sys::types::*;
use pam_sys::wrapped;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;
use std::slice;

fn string_from_ptr(ptr: *const c_char) -> Result<String, PamReturnCode> {
    let s = unsafe { CStr::from_ptr(ptr) };
    s.to_owned().into_string().map_err(|_| SYSTEM_ERR)
}

pub fn pam_get_args(argc: c_int, argv: *const *const c_char) -> Result<Vec<String>, PamReturnCode> {
    unsafe {
        let args = slice::from_raw_parts(argv, argc as usize);
        args.iter()
            .map(|i| string_from_ptr(*i))
            .collect::<Result<_, _>>()
            .map_err(|_| SYSTEM_ERR)
    }
}

pub fn pam_get_user(pamh: &PamHandle, prompt: &str) -> Result<String, PamReturnCode> {
    let mut user: *const c_char = ptr::null();
    let prompt = CString::new(prompt).map_err(|_| SYSTEM_ERR)?;
    match wrapped::get_user(pamh, &mut user, prompt.as_ptr()) {
        SUCCESS => Ok(string_from_ptr(user)?),
        e => Err(e),
    }
}

fn pam_get_item<T>(pamh: &PamHandle, itype: PamItemType) -> Result<Option<&T>, PamReturnCode> {
    let mut ptr: *const c_void = std::ptr::null();
    match pam_sys::wrapped::get_item(pamh, itype, &mut ptr) {
        SUCCESS => Ok(unsafe { ptr.cast::<T>().as_ref() }),
        e => Err(e),
    }
}

pub fn pam_get_string(pamh: &PamHandle, itype: PamItemType) -> Result<String, PamReturnCode> {
    let s = pam_get_item(pamh, itype)?;
    let s = s.ok_or(SYSTEM_ERR)?;
    string_from_ptr(s)
}

pub fn pam_prompt(
    pamh: &PamHandle,
    style: PamMessageStyle,
    msg: &str,
) -> Result<Option<String>, PamReturnCode> {
    let conv: Option<&PamConversation> = pam_get_item(pamh, PamItemType::CONV)?;
    let conv = conv.ok_or(SYSTEM_ERR)?;
    let callback = conv.conv.ok_or(SYSTEM_ERR)?;
    let msg = CString::new(msg).map_err(|_| SYSTEM_ERR)?;
    let mut msg: *mut PamMessage = &mut PamMessage {
        msg: msg.as_ptr(),
        msg_style: style as i32,
    };
    let mut resp: *mut PamResponse = std::ptr::null_mut();
    match PamReturnCode::from(callback(1, &mut msg, &mut resp, conv.data_ptr)) {
        SUCCESS => (),
        e => return Err(e),
    }
    unsafe {
        let resp = resp.as_ref().ok_or(SYSTEM_ERR)?;
        match resp.resp.as_ref() {
            Some(s) => Ok(Some(string_from_ptr(s)?)),
            None => Ok(None),
        }
    }
}
