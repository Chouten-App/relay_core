#![no_std]

extern crate alloc;
extern crate wee_alloc;

pub mod host;
pub mod types;
pub mod macros;
pub mod traits;

// pub mod export_macro;

use core::alloc::Layout;
use core::panic::PanicInfo;
use crate::host::log::log;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn grow_memory(pages: u32) -> i32 {
    unsafe {
        // memory index 0 (default memory)
        grow_mem(pages.try_into().unwrap())
    }
}

#[cfg(target_arch = "wasm32")]
pub unsafe fn grow_mem(pages: i32) -> i32 {
    core::arch::wasm32::memory_grow(0, pages as usize).try_into().unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
pub unsafe fn grow_mem(_pages: i32) -> i32 {
    // Not supported on non-wasm32 targets
    -1
}

// Exported alloc function for host calls
#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 8).unwrap();
    unsafe {
        alloc::alloc::alloc(layout)
    }
}

// Exported dealloc function for host calls
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, 8).unwrap();
    unsafe {
        alloc::alloc::dealloc(ptr, layout);
    }
}

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log(&alloc::format!(
            "PANIC at {}:{}:{}: {}",
            location.file(),
            location.line(),
            location.column(),
            info.message()
        ));
    } else {
        log("PANIC occurred, but no location information available");
    }
    // calling loop {} will hang the app
    // calling wasm32::unreachable instead
    core::arch::wasm32::unreachable()
}

unsafe extern "C" {
    pub fn log_host(ptr: *const u8, len: u32);
    pub fn request_host(url_ptr: *const u8, url_len: u32, method: u32) -> u32;
}
