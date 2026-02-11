use alloc::{string::String, vec::Vec};
use crate::types::{ChoutenError, DiscoverSection};

pub trait Source {
    fn discover(&self) -> Result<Vec<DiscoverSection>, ChoutenError>;
}

pub trait Tracker {
    fn auth_url(&self, client_id: &str, redirect_uri: &str, state: &str) -> String;

    fn handle_callback(&mut self, code: &str) -> Result<(), ChoutenError>;

    fn refresh_token(&mut self) -> Result<(), ChoutenError>;

    /// Example tracker actions
    fn update_progress(&self, media_id: &str, progress: i32) -> Result<(), ChoutenError>;
    // fn fetch_user(&self) -> Result<User, ChoutenError>;
    
    fn discover(&self) -> Result<Vec<DiscoverSection>, ChoutenError>;
}
