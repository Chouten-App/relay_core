use alloc::vec::Vec;

use crate::ResponseInfo;

use super::node::HtmlNode;

#[repr(transparent)]
#[derive(Clone)]
pub struct HtmlDocument {
    pub id: u32,
}

impl HtmlDocument {
    pub fn parse(html: &str) -> Self {
        let id = unsafe { super::html_parse_host(html.as_ptr() as u32, html.len() as u32) };
        Self { id }
    }

    pub fn query_selector(&self, selector: &str) -> Option<HtmlNode> {
        let node_id = unsafe { super::html_query_selector_host(self.id, selector.as_ptr() as u32, selector.len() as u32) };
        if node_id == 0 { None } else { Some(HtmlNode { id: node_id }) }
    }

    pub fn query_selector_all(&self, selector: &str) -> Vec<HtmlNode> {
        let mut len: u32 = 0;

        let ptr = unsafe {
            super::html_query_selector_all_host(
                self.id,
                selector.as_ptr() as u32,
                selector.len() as u32,
                &mut len,
            )
        };

        if ptr == 0 || len == 0 {
            return Vec::new();
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