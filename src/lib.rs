#![feature(doc_cfg)]

//! Darwin Notify API bindings for Rust.
//!
//! Find the API docs on [official Apple docs](https://developer.apple.com/documentation/darwinnotify)
//!

use std::ffi::CString;

use block::ConcreteBlock;

#[cfg(not(feature = "sys"))]
mod sys;

#[cfg(any(feature = "sys"))]
/// Contains raw C bindings to Darwin Notify API.
///
/// This module is only availabe when `sys` feature is enabled.
pub mod sys;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRunLoopRun();
}

/// Errors returned by Darwin Notify API
#[derive(Debug)]
#[repr(u32)]
pub enum NotifyError {
    InvalidName = 1,
    InvalidToken,
    InvalidPort,
    InvalidFile,
    InvalidSignal,
    InvalidRequest,
    NotAuthorized,
    OptDisabled,
    ServerNotFound,
    NullInput,

    Failed = 1000000,

    Unknown = u32::MAX,
}

impl NotifyError {
    fn from_u32(code: u32) -> Self {
        match code {
            1 => Self::InvalidName,
            2 => Self::InvalidToken,
            3 => Self::InvalidPort,
            4 => Self::InvalidFile,
            5 => Self::InvalidSignal,
            6 => Self::InvalidRequest,
            7 => Self::NotAuthorized,
            8 => Self::OptDisabled,
            9 => Self::ServerNotFound,
            10 => Self::NullInput,
            1000000 => Self::Failed,

            code @ _ => {
                tracing::error!(
                "This is a bug, please report on github. {code} should've never been the status."
            );

                NotifyError::Unknown
            }
        }
    }
}

impl std::error::Error for NotifyError {}

impl std::fmt::Display for NotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => f.write_str("Darwin Notify Error: Unknown error, this is most certainly a bug. Please report issue on github."),
            err @ _ => f.write_str(&format!("Darwin Notify Error: {err:?}"))
        }
    }
}

pub type NResult<T> = Result<T, NotifyError>;

macro_rules! ns_result {
    ($e: expr) => {
        match $e {
            0 => Ok(()),
            code @ _ => Err(NotifyError::from_u32(code)),
        }
    };
}

/// Post a notification for a name
///
/// # Example
/// ```
/// fn main() {
///     darwin_notify::notify("tech.subcom.darwin-notify")
/// }
/// ```
pub fn notify_post(name: &str) -> NResult<()> {
    let name = CString::new(name).unwrap();
    ns_result!(unsafe { sys::notify_post(name.as_ptr()) })
}

/// Subscribe to receive notification for a name.
///
/// This function uses `notify_register_dispatch` with current dispatch queue to recieve notifications.
///
/// If you want more control, enable the `sys` feature and use [sys::notify_register_dispatch].
///
/// # Example
/// ```
/// fn main() {
///     darwin_notify::notify_register("tech.subcom.darwin-notify", |token| { println!("Got a notification: {token}") })
/// }
/// ```
pub fn notify_register<F>(name: &str, cb: F) -> NResult<i32>
where
    F: Fn(std::ffi::c_int) + 'static,
{
    let name = CString::new(name).unwrap();

    let mut token = 0;
    let dque = unsafe { sys::dispatch_get_current_queue() };

    let block = ConcreteBlock::new(move |token: i32| cb(token)).copy();

    match unsafe {
        sys::notify_register_dispatch(
            name.as_ptr(),
            &mut token as _,
            dque,
            &*block as *const _ as _,
        )
    } {
        0 => Ok(token),
        code @ _ => Err(NotifyError::from_u32(code)),
    }
}

/// Suspend delivery of notifcations
pub fn notify_suspend(token: std::ffi::c_int) -> NResult<()> {
    ns_result!(unsafe { sys::notify_suspend(token) })
}

/// Set or get a state value associated with a notification token.
pub fn notify_set_state(token: std::ffi::c_int, state: u64) -> NResult<()> {
    ns_result!(unsafe { sys::notify_set_state(token, state) })
}

/// Get the 64-bit integer state value.
pub fn notify_get_state(token: std::ffi::c_int) -> NResult<u64> {
    let mut state = 0;

    match unsafe { sys::notify_get_state(token, &mut state as _) } {
        0 => Ok(state),
        code @ _ => Err(NotifyError::from_u32(code)),
    }
}

/// Check if any notifications have been posted.
pub fn notify_check(token: std::ffi::c_int) -> NResult<bool> {
    let mut check = 0;

    match unsafe { sys::notify_check(token, &mut check as _) } {
        0 => Ok(check == 1),
        code @ _ => Err(NotifyError::from_u32(code)),
    }
}

/// Cancel notification and free resources associated with a notification token.
pub fn notify_cancel(token: std::ffi::c_int) -> NResult<()> {
    ns_result!(unsafe { sys::notify_cancel(token) })
}

/// Removes one level of suspension for a token previously suspended by a call to notify_suspend
pub fn notify_resume(token: std::ffi::c_int) -> NResult<()> {
    ns_result!(unsafe { sys::notify_resume(token) })
}
