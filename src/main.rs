#![no_std]
#![no_main]
extern crate alloc;

mod panic;

#[global_allocator]
static ALLOC: dlmalloc::GlobalDlmalloc = dlmalloc::GlobalDlmalloc;

use core::ffi::c_int;
use core::ptr;

use cstr::cstr;
use vc6_sys as sys;

#[no_mangle]
pub extern "system" fn WinMain() -> c_int {
    let message = cstr!("Hello from Rust!");
    let title = cstr!("Crabs");

    unsafe {
        sys::MessageBoxA(ptr::null_mut(),
            message.as_ptr(),
            title.as_ptr(),
            sys::MB_OK | sys::MB_ICONASTERISK);
    }

    panic!("oh no!");
}
