use core::error::Error;

use crate::RESPONSE_LEN;
use crate::RESPONSE_PTR;
use crate::ResponseInfo;
use crate::log_host;
use crate::store_response;
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


impl Request {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            headers: Vec::new(),
            method: RequestMethod::GET
        }
    }

    pub fn send(self) -> Result<Response, RequestError> {
        let struct_ptr = unsafe { 
            request_host(self.url.as_ptr(), self.url.len() as u32, self.method as u32) 
        };
        
        if struct_ptr == 0 {
            crate::log("Request failed");
            return Err(RequestError::UNKNOWN);
        }
        
        let info: &ResponseInfo = unsafe {
            &*(struct_ptr as *const ResponseInfo)
        };
        
        let body_ptr = info.ptr;
        let body_len = info.len;
        
        if body_len == 0 || body_ptr == 0 {
            crate::log("Invalid response");
            return Err(RequestError::UNKNOWN);
        }
        
        crate::log("Got response info");
        
        let bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(body_ptr as *const u8, body_len as usize)
        };
        
        if bytes[0] != b'{' {
            crate::log("Not JSON");
            return Err(RequestError::UNKNOWN);
        }
        
        crate::log("Bytes look like JSON");
        
        // Convert bytes to string
        let json_str = core::str::from_utf8(bytes)
            .map_err(|_| {
                crate::log("UTF8 conversion failed");
                RequestError::InvalidUtf8
            })?;
        
        crate::log("UTF8 conversion OK");
        
        if json_str.is_empty() {
            crate::log("String is empty");
            return Err(RequestError::UNKNOWN);
        }
        
        if json_str.len() > 50 {
            crate::log(&json_str[..50]);
        } else {
            crate::log(json_str);
        }
        
        crate::log("About to parse JSON");
        
        let http_resp: Result<(HttpResponseJson, usize), serde_json_core::de::Error> = 
            serde_json_core::from_str(json_str);

        match http_resp {
            Ok((http_resp, _)) => {
                crate::log("Parse success");
                Ok(Response {
                    status_code: http_resp.status_code,
                    body: http_resp.body.to_string(),
                })
            }
            Err(e) => {
                crate::log("Parse error");
                match e {
                    serde_json_core::de::Error::EofWhileParsingValue => crate::log("EOF error"),
                    serde_json_core::de::Error::ExpectedSomeValue => crate::log("Expected value"),
                    _ => crate::log("Other parse error"),
                }
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