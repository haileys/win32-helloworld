use core::ffi::CStr;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr;
use alloc::string::String;

use cstr::cstr;
use vc6_sys as sys;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // format panic info
    let mut msg = String::new();
    let _ = write!(msg, "{}", info);

    // null-terminate it and take a cstring of it
    let msg = unsafe {
        msg += "\0";
        CStr::from_bytes_with_nul_unchecked(msg.as_bytes())
    };

    // show message box to user
    unsafe {
        sys::MessageBoxA(ptr::null_mut(),
            msg.as_ptr(),
            cstr!("Panic").as_ptr(),
            sys::MB_OK | sys::MB_ICONSTOP);
    }

    // exit
    unsafe { sys::FatalExit(1); }
    unreachable!()
}
