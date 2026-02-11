use alloc::{borrow::ToOwned, string::String};

#[derive(Copy, Clone)]
pub enum SettingValue {
    Bool(bool),
    Int(i32),
    Str(&'static str),
}

// A single "setting" struct for demonstration
pub struct Setting {
    pub group: &'static str,
    pub key: &'static str,
    pub value: SettingValue,
}

pub trait FromSettingValue<'a>: Sized {
    fn from_setting_value_bytes(bytes: &'a [u8]) -> Option<Self>;
}

impl<'a> FromSettingValue<'a> for String {
    fn from_setting_value_bytes(bytes: &'a [u8]) -> Option<Self> {
        core::str::from_utf8(bytes).ok().map(|s| s.to_owned())
    }
}

impl<'a> FromSettingValue<'a> for bool {
    fn from_setting_value_bytes(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() != 1 { return None; }
        Some(bytes[0] != 0)
    }
}

impl<'a> FromSettingValue<'a> for i32 {
    fn from_setting_value_bytes(bytes: &'a [u8]) -> Option<Self> {
        if bytes.len() != 4 { return None; }
        let arr: [u8; 4] = bytes.try_into().ok()?;
        Some(i32::from_le_bytes(arr))
    }
}

// Generic getter
pub fn get_setting_in_group<'a, T>(group: &str, key: &str) -> Option<T>
where
    T: FromSettingValue<'a>,
{
    let mut out_ptr: u32 = 0;

    // Call the host function
    let len = unsafe {
        get_setting_in_group_host(
            group.as_ptr(),
            group.len(),
            key.as_ptr(),
            key.len(),
            &mut out_ptr,
        )
    };

    if len == 0 || out_ptr == 0 {
        return None;
    }

    unsafe {
        let slice = core::slice::from_raw_parts(out_ptr as *const u8, len as usize);

        // Delegate conversion to T
        T::from_setting_value_bytes(slice)
    }
}

unsafe extern "C" {
    fn get_setting_in_group_host(group_ptr: *const u8, group_len: usize, key_ptr: *const u8, key_len: usize, out_ptr: *mut u32) -> u32;
}