// runtime_support.rs

/// Serialize `value` into `buf` as null-terminated JSON.
/// Returns the buffer pointer cast to usize — caller converts to i32 for ABI.
///
/// Return type is usize here (not i32) so the proc-macro side can cast
/// cleanly; the extern "C" wrapper does the final `as i32`.
#[inline(never)]
pub unsafe fn __chouten_write_json<T: serde::Serialize>(
    value: &T,
    buf: &mut [u8; 65536],
) -> usize {
    match serde_json_core::to_slice(value, &mut buf[..65535]) {
        Ok(len) => {
            buf[len] = 0;
        }
        Err(_) => {
            const ERR: &[u8] = b"{\"Err\":\"overflow\"}\0";
            buf[..ERR.len()].copy_from_slice(ERR);
        }
    }
    buf.as_ptr() as usize
}

pub fn __chouten_read_str_arg() -> alloc::string::String {
    unsafe extern "C" {
        fn host_get_string_arg() -> i32; // i32, not *const u8
    }
    unsafe {
        let ptr = host_get_string_arg() as usize as *const u8;
        let mut len = 0usize;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        core::str::from_utf8_unchecked(
            core::slice::from_raw_parts(ptr, len)
        ).into()
    }
}

pub fn serialize_to_json<T: serde::Serialize>(value: &T) -> alloc::vec::Vec<u8> {
    // use serde_json_core or your existing serializer
    let mut buf = alloc::vec![0u8; 65536];
    let len = serde_json_core::to_slice(value, &mut buf).unwrap_or(0);
    buf.truncate(len);
    buf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn chouten_free_result(struct_ptr: u32) {
    let json_struct = alloc::boxed::Box::from_raw(struct_ptr as *mut [u32; 2]);
    let json_ptr = json_struct[0] as *mut u8;
    let json_len = json_struct[1] as usize;
    drop(alloc::vec::Vec::from_raw_parts(json_ptr, json_len, json_len));
    // json_struct (the Box) is dropped here automatically
}