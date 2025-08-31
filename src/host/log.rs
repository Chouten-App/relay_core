use crate::log_host;
use alloc::string::String;

use core::fmt::{self, Write};

pub fn log<S: Into<String>>(s: S) {
    let owned = s.into();
    unsafe {
        log_host(owned.as_ptr(), owned.len().try_into().unwrap());
    }
    // owned is dropped *after* the call, so pointer is valid during log_host
}

pub struct FixedBuffer<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> FixedBuffer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.pos]).unwrap_or("???")
    }
}

impl<'a> Write for FixedBuffer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buf.len().saturating_sub(self.pos);
        let copy_len = bytes.len().min(remaining);
        self.buf[self.pos..self.pos + copy_len].copy_from_slice(&bytes[..copy_len]);
        self.pos += copy_len;
        Ok(())
    }
}

#[macro_export]
macro_rules! log_fmt {
    ($($arg:tt)*) => {{
        let mut buf = [0u8; 128];
        let mut writer = $crate::FixedBuffer::new(&mut buf);
        core::fmt::write(&mut writer, format_args!($($arg)*)).unwrap();
        crate::log(writer.as_str());
    }};
}