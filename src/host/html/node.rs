use alloc::string::ToString;

use crate::ResponseInfo;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct HtmlNode {
    pub id: u32,
}

impl HtmlNode {
    pub fn text(&self) -> alloc::string::String {
        let mut out_len: u32 = 0;
        let struct_ptr = unsafe { super::html_node_text_host(self.id, &mut out_len) };
        if struct_ptr == 0 {
            crate::log("Request failed");
            return "".to_string();
        }
        
        let info: &ResponseInfo = unsafe {
            &*(struct_ptr as *const ResponseInfo)
        };
        
        let body_ptr = info.ptr;
        let body_len = info.len;

        unsafe {
            let slice = core::slice::from_raw_parts(body_ptr as *const u8, body_len as usize);
            alloc::string::String::from_utf8_lossy(slice).into_owned()
        }
    }

    pub fn attr(&self, attr: &str) -> alloc::string::String {
        let mut out_ptr: u32 = 0;

        let attr_ptr = attr.as_ptr() as u32;
        let attr_len = attr.len() as u32;

        let struct_ptr = unsafe { super::html_node_attr_host(self.id, attr_ptr, attr_len, &mut out_ptr) };
        if struct_ptr == 0 {
            crate::log("Request failed");
            return "".to_string();
        }
    
        let info: &ResponseInfo = unsafe {
            &*(struct_ptr as *const ResponseInfo)
        };
        
        let body_ptr = info.ptr;
        let body_len = info.len;

        unsafe {
            let slice = core::slice::from_raw_parts(body_ptr as *const u8, body_len as usize);
            alloc::string::String::from_utf8_lossy(slice).into_owned()
        }
    }

    pub fn query_selector(&self, selector: &str) -> Option<HtmlNode> {
        let node_id = unsafe { super::html_node_query_selector_host(self.id, selector.as_ptr(), selector.len()) };
        if node_id == 0 { None } else { Some(HtmlNode { id: node_id }) }
    }

    pub fn query_selector_all(&self, selector: &str) -> alloc::vec::Vec<HtmlNode> {
        let mut out_ptr: u32 = 0;
        let len = unsafe { super::html_node_query_selector_all_host(self.id, selector.as_ptr(), selector.len() as u32, &mut out_ptr) };

        if len == 0 || out_ptr == 0 {
            return alloc::vec::Vec::new();
        }

        unsafe {
            let slice = core::slice::from_raw_parts(out_ptr as *const HtmlNode, len as usize);
            slice.to_vec()
        }
    }
}
