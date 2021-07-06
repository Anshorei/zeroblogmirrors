pub mod datetime;
pub mod cached_file;

use std::borrow::Cow;
use std::boxed::Box;
use ammonia::UrlRelative;
use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag, html};
use log::*;

pub fn markdown(content: &str) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);

  let parser = Parser::new_ext(content, options)
    .map(|event| match event {
      Event::Start(_) | Event::End(_) => image_to_video(event),
      _ => event,
    });
  let mut unsafe_html = String::new();
  html::push_html(&mut unsafe_html, parser);

  ammonia::Builder::default()
    .add_tags(&["video", "audio"])
    .add_tag_attributes("video", &["src", "controls"])
    .add_tag_attributes("audio", &["src", "controls"])
    .url_relative(UrlRelative::Custom(Box::new(evaluate)))
    .clean(&*unsafe_html)
    .to_string()
}

fn image_to_video(event: Event) -> Event {
  match event.clone() {
    Event::Start(Tag::Image(link, dest, title)) => {
      if dest.ends_with(".mp4") {
        Event::Html(CowStr::from(format!("<video controls=\"controls\" src=\"{}\" title>Your browser does not support the video tag.<span hidden>", dest)))
      } else if dest.ends_with(".mp3") {
        Event::Html(CowStr::from(format!("<audio controls=\"controls\" src=\"{}\" title>Your browser does not support the audio tag.<span hidden>", dest)))
      } else {
        event
      }
    }
    Event::End(Tag::Image(link, dest, title)) => {
      if dest.ends_with(".mp4") {
        Event::Html(CowStr::Borrowed("</span></video>"))
      } else if dest.ends_with(".mp3") {
        Event::Html(CowStr::Borrowed("</span></audio>"))
      } else {
        event
      }
    }
    _ => event,
  }
}

fn is_absolute_path(url: &str) -> bool {
  let u = url.as_bytes();
  // `//a/b/c` is "protocol-relative", meaning "a" is a hostname
  // `/a/b/c` is an absolute path, and what we want to do stuff to.
  u.get(0) == Some(&b'/') && u.get(1) != Some(&b'/')
}

fn evaluate(url: &str) -> Option<Cow<str>> {
  if is_absolute_path(url) {
    // TODO: handle ZeroNet urls
    Some(Cow::Borrowed(url))
  } else {
    // let url = url.split_once('/').map(|url| url.1).unwrap_or("");
    Some(Cow::Owned(String::from("/") + url))
  }
}
