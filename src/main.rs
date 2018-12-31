#![feature(lang_items, core_intrinsics, start)]
#![no_std]
use core::intrinsics;
use core::panic::PanicInfo;

// Pull in the system libc library
extern crate libc;

// Entry point for this program.
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        libc::printf("Hello, world!\n\0".as_ptr() as *const _);
    }
    0
}

// Required language items
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}
