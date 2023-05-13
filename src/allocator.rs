use alloc::alloc::{GlobalAlloc, Layout};
use core::cmp;
use core::mem;
use core::ptr;

use vc6_sys as sys;

#[global_allocator]
static ALLOC: Win32Alloc = Win32Alloc;

pub struct Win32Alloc;

unsafe impl GlobalAlloc for Win32Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let header_size = mem::size_of::<sys::HGLOBAL>();
        let align = cmp::min(header_size, layout.align());
        let size = header_size + layout.size() + align;

        let hmem = sys::GlobalAlloc(0, u32::try_from(size).expect("usize -> u32"));
        if hmem == ptr::null_mut() {
            return ptr::null_mut();
        }

        let pmem = sys::GlobalLock(hmem) as *mut u8;

        let ptr = pmem.add(header_size);
        let ptr = ptr.add(ptr.align_offset(align));

        let handle_ptr = ptr.sub(header_size);
        ptr::write(handle_ptr as *mut sys::HGLOBAL, hmem);

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let header_size = mem::size_of::<sys::HGLOBAL>();
        let handle_ptr = ptr.sub(header_size);
        let hmem = ptr::read(handle_ptr as *mut sys::HGLOBAL);
        sys::GlobalFree(hmem);
    }
}
