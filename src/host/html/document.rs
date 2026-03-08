use crate::ResponseInfo;

use super::node::HtmlNode;

#[repr(transparent)]
#[derive(Clone)]
pub struct HtmlDocument {
    pub id: u32,
}

impl HtmlDocument {
    pub fn parse(html: &str) -> Self {
        let id = unsafe { super::html_parse_host(html.as_ptr(), html.len()) };
        Self { id }
    }

    pub fn query_selector(&self, selector: &str) -> Option<HtmlNode> {
        let node_id = unsafe { super::html_query_selector_host(self.id, selector.as_ptr(), selector.len()) };
        if node_id == 0 { None } else { Some(HtmlNode { id: node_id }) }
    }

    pub fn query_selector_all(&self, selector: &str) -> alloc::vec::Vec<HtmlNode> {
        let mut out_ptr: u32 = 0;
        let struct_ptr = unsafe { super::html_query_selector_all_host(self.id, selector.as_ptr(), selector.len() as u32, &mut out_ptr) };

        if struct_ptr == 0 {
            crate::log("Request failed");
            return alloc::vec::Vec::new();
        }
        
        let info: &ResponseInfo = unsafe {
            &*(struct_ptr as *const ResponseInfo)
        };
        
        let ptr = info.ptr;
        let len = info.len;
        
        if len == 0 || ptr == 0 {
            crate::log("Invalid response");
            return alloc::vec::Vec::new();
        }

        unsafe {
            let slice = core::slice::from_raw_parts(ptr as *const HtmlNode, len as usize);
            slice.to_vec()
        }
    }
}

/*
impl Drop for HtmlDocument {
    fn drop(&mut self) {
        unsafe { super::html_free_doc_host(self.id) }
    }
}
*/