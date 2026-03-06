pub mod document;
pub mod node;

pub use document::HtmlDocument;
pub use node::HtmlNode;

unsafe extern "C" {
    fn html_parse_host(ptr: *const u8, len: usize) -> u32;
    fn html_free_doc_host(doc_id: u32);

    fn html_query_selector_host(doc_id: u32, ptr: *const u8, len: usize) -> u32;
    fn html_query_selector_all_host(doc_id: u32, sel_ptr: *const u8, sel_len: u32, out_ptr: *mut u32) -> u32;
    
    fn html_node_query_selector_host(doc_id: u32, ptr: *const u8, len: usize) -> u32;
    fn html_node_query_selector_all_host(doc_id: u32, sel_ptr: *const u8, sel_len: u32, out_ptr: *mut u32) -> u32;
    fn html_node_text_host(node_id: u32, out_len: *mut u32) -> u32;
    fn html_node_attr_host(node_id: u32, attr_ptr: u32, attr_len: u32, out_ptr: *mut u32) -> u32;
}