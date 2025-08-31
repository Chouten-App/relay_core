use crate::types::RequestMethod;
use crate::types::HttpResponse;
use crate::types::Response;
use alloc::string::String;
use crate::request_host;
use alloc::vec::Vec;
use crate::alloc::string::ToString;
use crate::types::RequestError;

pub struct Request {
    url: String,
    headers: Vec<(String, String)>,
    method: RequestMethod
}

impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            headers: Vec::new(),
            method: RequestMethod::GET
        }
    }

    pub fn send(self) -> Result<Response, RequestError> {
        let ptr = unsafe { request_host(self.url.as_ptr(), self.url.len() as u32, self.method as u32) };

        // Dereference the pointer to HttpResponse safely (inside unsafe block)
        let resp: &HttpResponse = unsafe { &*(ptr as *const HttpResponse) };

        // Convert raw pointer and length to Rust String
        let body = if resp.body_ptr != 0 && resp.body_len > 0 {
            unsafe {
                let slice = core::slice::from_raw_parts(resp.body_ptr as *const u8, resp.body_len as usize);
                String::from_utf8_lossy(slice).into_owned()
            }
        } else {
            String::new()
        };

        Ok(
            Response {
                status_code: resp.status_code,
                body,
            }
        )
    }

    pub fn set_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn set_method(mut self, method: RequestMethod) -> Self {
        self.method = method;
        self
    }
}

pub fn request(url: &str, method: RequestMethod) -> Response {
    let ptr = unsafe { request_host(url.as_ptr(), url.len() as u32, method as u32) };

    // Dereference the pointer to HttpResponse safely (inside unsafe block)
    let resp: &HttpResponse = unsafe { &*(ptr as *const HttpResponse) };

    // Convert raw pointer and length to Rust String
    let body = if resp.body_ptr != 0 && resp.body_len > 0 {
        unsafe {
            let slice = core::slice::from_raw_parts(resp.body_ptr as *const u8, resp.body_len as usize);
            String::from_utf8_lossy(slice).into_owned()
        }
    } else {
        String::new()
    };

    Response {
        status_code: resp.status_code,
        body,
    }
}

