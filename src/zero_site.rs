use std::fs;
use std::sync::{Arc, Mutex};
use maud::{html, PreEscaped, Markup};
use std::path::PathBuf;

use crate::schemas::zeroblog;
use crate::utils;
use log::*;

#[derive(Default)]
pub struct SiteData {
  pub content: zeroblog::Content,
  pub data: zeroblog::ZeroBlog,
}

impl SiteData {
  pub fn load_content(&mut self, path: String) {
    debug!("Loading content from {:?}", path);
    let contents = fs::read_to_string(path).unwrap();
    self.content = serde_json::from_str(&contents).unwrap();
  }

  pub fn load_data(&mut self, path: String) {
    debug!("Loading data from {:?}", path);
    let contents = fs::read_to_string(path).unwrap();
    self.data = serde_json::from_str(&contents).unwrap();
  }

  pub fn index(&self) -> Markup {
    html! {
      h1 { (self.data.title) }
      div { (PreEscaped(utils::markdown(&self.data.description))) }
      div {
        @for post in &self.data.post {
          h2 { (post.title) }
          div { (PreEscaped(utils::markdown(&post.short_body()))) }
          a href=(format!("/post/{}", post.post_id)) { "Read more ->" }
        }
      }
    }
  }

  pub fn blog_post(&self, post_id: usize) -> Markup {
    let post = self.data.post.iter().find(|p| p.post_id == post_id);
    html! {
      h1 { (self.data.title) }
      @if post_id > 1 {
        a href=(format!("/post/{}", post_id-1)) { "<- Prev" }
      }
      { " " }
      @if post_id < self.data.next_post_id-1 {
        a href=(format!("/post/{}", post_id+1)) { "Next ->" }
      }
      @if let Some(post) = post {
        h2 { (post.title) }
        p { (utils::datetime::format_timestamp(post.date_published as i64)) }
        div { (PreEscaped(utils::markdown(&post.body))) }
      } @else {
        h2 { "404 Not Found" }
      }
    }
  }
}
