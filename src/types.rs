use alloc::string::String;
use serde::{Serialize, Deserialize};
use crate::alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoutenError {
    Network {
        url: String,
        message: String,
    },

    HtmlParse {
        selector: String,
        message: String,
    },

    Host {
        function: String,
        message: String,
    },

    Module {
        message: String,
    },
}

impl ChoutenError {
    pub fn network(url: &str, msg: &str) -> Self {
        Self::Network { url: url.to_string(), message: msg.to_string() }
    }

    pub fn html(selector: &str, msg: &str) -> Self {
        Self::HtmlParse { selector: selector.to_string(), message: msg.to_string() }
    }

    pub fn host(func: &str, msg: &str) -> Self {
        Self::Host { function: func.to_string(), message: msg.to_string() }
    }

    pub fn module(msg: &str) -> Self {
        Self::Module { message: msg.to_string() }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct HttpResponseJson<'a> {
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    pub body: &'a str,
}

pub struct HttpResponse {
    pub status_code: u32,
    pub body_ptr: *const u8,
    pub body_len: usize,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum RequestMethod {
    GET = 0,
    POST = 1,
    PUT = 2,
    DELETE = 3
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum RequestError {
    TIMEOUT,
    UNKNOWN,
    InvalidUtf8
}

impl From<RequestError> for ChoutenError {
    fn from(err: RequestError) -> Self {
        match err {
            RequestError::TIMEOUT => ChoutenError::network("", "Request timed out"),
            RequestError::UNKNOWN => ChoutenError::network("", "Unknown request error"),
            RequestError::InvalidUtf8 => ChoutenError::network("", "Invalid UTF8")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u32,
    pub body: String,
}

#[repr(u32)]
#[derive(Serialize, Deserialize)]
pub enum DiscoverSectionType {
    CAROUSEL,
    LIST
}

#[derive(Serialize, Deserialize)]
pub struct DiscoverSection {
    pub title: String,
    pub section_type: DiscoverSectionType,
    pub list: Vec<DiscoverData>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Titles {
    pub primary: String,
    pub secondary: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Label {
    pub text: String,
    pub color: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscoverData {
    pub url: String,
    pub titles: Titles,
    pub poster: String,
    pub banner: Option<String>,
    pub description: String,
    pub label: Label,
    pub indicator: Option<String>,
    pub current: Option<u32>,
    pub total: Option<u32>
}