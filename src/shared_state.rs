use std::sync::{Arc, Mutex};


#[derive(Default)]
pub struct SharedState {
  pub content: crate::schemas::zeroblog::Content,
  pub data: crate::schemas::zeroblog::ZeroBlog,
}
