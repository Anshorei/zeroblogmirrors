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

  pub fn render(&self, inner: Markup) -> Markup {
    html! {
      link rel="stylesheet" type="text/css" href="/css";
      body class="loaded page-main" {
        div class="left" {
          div class="trigger" {
            a class="nolink" href="/" {
              h1 class="title" { (self.data.title) }
            }
            div { (PreEscaped(utils::markdown(&self.data.description))) }
          }
        }
        div class="right" {
          (inner)
        }
      }
    }
  }

  pub fn index(&self) -> Markup {
    let inner = html! {
      div class="posts" {
        @for post in &self.data.post {
          div class="post" {
            h2 class="title" { (post.title) }
            p class="details" {
              span { "on " }
              span { (utils::datetime::format_date(post.date_published as i64)) }
            }
            div class="body" {
              (PreEscaped(utils::markdown(&post.short_body())))
            }
            a href=(format!("/post/{}", post.post_id)) { "Read more ->" }
          }
        }
      }
    };
    self.render(inner)
  }

  pub fn blog_post(&self, post_id: usize) -> Markup {
    let post = self.data.post.iter().find(|p| p.post_id == post_id);
    let inner = html! {
      div style="display: flex" {
        @if post_id > 1 {
          div {
            a href=(format!("/post/{}", post_id-1)) { "<- Prev" }
          }
        }
        div style="flex-grow: 1; text-align: center" {
          a href="/" {
            "Home"
          }
        }
        @if post_id < self.data.next_post_id-1 {
          div {
            a href=(format!("/post/{}", post_id+1)) { "Next ->" }
          }
        }
      }
      @if let Some(post) = post {
        div class="post" {
          h2 { (post.title) }
          p class="details" {
            span { "on " }
            span { (utils::datetime::format_date(post.date_published as i64)) }
          }
          div class="body" {
            (PreEscaped(utils::markdown(&post.body)))
          }
        }
      } @else {
        h2 { "404 Not Found" }
      }
    };
    self.render(inner)
  }
}
