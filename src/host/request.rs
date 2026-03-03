use crate::log_host;
use crate::types::ChoutenError;
use crate::types::HttpResponseJson;
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

struct RelayResponse {
    pub ptr: u32,
    pub len: u32
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

        crate::log("Received");
        
        // Dereference the pointer to RelayResponse
        let relay_resp: &RelayResponse = unsafe { &*(ptr as *const RelayResponse) };
        
        // Convert RelayResponse to byte slice
        let bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(relay_resp.ptr as *const u8, relay_resp.len as usize)
        };
        crate::log("RelayResponse created");
        
        // Convert bytes to string
        let json_str = core::str::from_utf8(bytes)
            .map_err(|_| RequestError::InvalidUtf8)?;
        crate::log("Json string created");
        crate::log(json_str);
        
        // Parse JSON string into HttpResponse
        let http_resp: Result<(HttpResponseJson, usize), serde_json_core::de::Error> = serde_json_core::from_str(json_str);

        match http_resp {
            Ok((http_resp, _)) => {
                Ok(Response {
                    status_code: http_resp.status_code,
                    body: http_resp.body.to_string(),
                })
            }
            Err(_) => {
                crate::log("Error");
                Err(RequestError::UNKNOWN)
            }
        }
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