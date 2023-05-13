use core::ffi::{c_size_t, c_int};

#[no_mangle]
pub unsafe extern "C" fn _memcpy(dst: *mut u8, src: *const u8, len: c_size_t) -> *mut u8 {
    core::ptr::copy_nonoverlapping(src, dst, len);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn _memmove(dst: *mut u8, src: *const u8, len: c_size_t) -> *mut u8 {
    core::ptr::copy(src, dst, len);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn _memcmp(mut a: *const u8, mut b: *const u8, len: c_size_t) -> c_int {
    while len > 0 {
        if *a < *b {
            return -1;
        }

        if *a > *b {
            return 1;
        }

        a = a.add(1);
        b = b.add(1);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn _memset(dst: *mut u8, val: u8, len: c_size_t) -> *mut u8 {
    core::ptr::write_bytes(dst, val,len);
    dst
}
