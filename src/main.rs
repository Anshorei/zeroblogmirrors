#![feature(proc_macro_hygiene, decl_macro)]
use log::*;
use rocket::{get, routes, Config, State, response::NamedFile};
use maud::{html, Markup, PreEscaped};
use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
use std::sync::mpsc::channel;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::{Path, PathBuf};

mod schemas;
mod args;
mod file_watcher;
mod shared_state;
mod utils;

use shared_state::SharedState;
use args::Args;

struct StateWrapper {
  shared_state: Arc<Mutex<SharedState>>,
  args: Args,
}

fn main() {
  let args = args::get_arguments().expect("Could not get required arguments");
  pretty_env_logger::init_timed();

  let (sender, receiver) = channel();
  let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();

  let shared_state = Arc::new(Mutex::new(SharedState::default()));
  let moved_state = shared_state.clone();
  let moved_args = args.clone();
  thread::spawn(|| file_watcher::watch(moved_args, watcher, receiver, moved_state));

  let mut config = Config::active().unwrap();
  config.set_port(args.rocket_port);
  let state = StateWrapper{shared_state, args};
  rocket::custom(config)
    .mount("/", routes![index, blog_post, static_file])
    .manage(state)
    .launch();
}

#[get("/")]
fn index(state: State<StateWrapper>) -> Markup {
  let state = state.shared_state.lock().unwrap();
  html! {
    h1 { (state.data.title) }
    div { (PreEscaped(utils::markdown(&state.data.description))) }
    div {
      @for post in &state.data.post {
        h2 { (post.title) }
        div { (PreEscaped(utils::markdown(&post.short_body()))) }
        a href=(format!("/post/{}", post.post_id)) { "Read more ->" }
      }
    }
  }
}

#[get("/post/<post_id>")]
fn blog_post(state: State<StateWrapper>, post_id: usize) -> Markup {
  let state = state.shared_state.lock().unwrap();
  let post = state.data.post.iter().find(|p| p.post_id == post_id);
  html! {
    h1 { (state.data.title) }
    @if post_id > 1 {
      a href=(format!("/post/{}", post_id-1)) { "<- Prev" }
    }
    { " " }
    @if post_id < state.data.next_post_id-1 {
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

#[get("/data/<path..>")]
fn static_file(state: State<StateWrapper>, path: PathBuf) -> Option<NamedFile> {
  let mut path = Path::new(&state.args.zeronet_path).join(&state.args.site_address).join("data").join(path);
  if path.is_dir() {
    path.push("index.html");
  }
  trace!("Data file request: {:?}", path);

  NamedFile::open(path).ok()
}
