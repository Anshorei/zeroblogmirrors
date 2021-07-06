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
use std::collections::HashMap;

mod schemas;
mod args;
mod file_watcher;
mod zero_site;
mod utils;

use zero_site::SiteData;
use args::Args;

use crate::utils::cached_file::CachedFile;

struct StateWrapper {
  sites: HashMap<String, Arc<Mutex<SiteData>>>,
  args: Args,
}

fn main() {
  let args = args::get_arguments().expect("Could not get required arguments");
  pretty_env_logger::init_timed();

  let (sender, receiver) = channel();
  let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();

  let mut sites = HashMap::new();
  args.site_addresses.iter().for_each(|address| {
    sites.insert(address.clone(), Arc::new(Mutex::new(SiteData::default())));
  });

  let moved_sites = sites.clone();
  let moved_args = args.clone();
  thread::spawn(|| file_watcher::watch(moved_args, watcher, receiver, moved_sites));

  let mut config = Config::active().unwrap();
  config.set_port(args.rocket_port);
  let state = StateWrapper{sites, args};
  rocket::custom(config)
    .mount("/", routes![index, static_data_file, static_font_file, blog_post, css])
    .manage(state)
    .launch();
}

#[get("/<address>")]
fn index(state: State<StateWrapper>, address: String) -> Option<Markup> {
  trace!("Request for index of {}", address);
  Some(state.sites.get(&address)?.lock().unwrap().index())
}

#[get("/<address>/css")]
fn css(state: State<StateWrapper>, address: String) -> Option<CachedFile> {
  trace!("Request for css of {}", address);
  if state.sites.contains_key(&address) {
    let path = Path::new(&state.args.zeronet_path).join(address).join("css/all.css");
    trace!("Passing {:?}", &path);
    return NamedFile::open(path).map(|nf| CachedFile::new(nf)).ok();
  }
  None
}

#[get("/<address>/post/<post_id>")]
fn blog_post(state: State<StateWrapper>, address: String, post_id: usize) -> Option<Markup> {
  trace!("Request for blog post {} of {}", post_id, address);
  Some(state.sites.get(&address)?.lock().unwrap().blog_post(post_id))
}

#[get("/<address>/data/<path..>")]
fn static_data_file(state: State<StateWrapper>, address: String, path: PathBuf) -> Option<NamedFile> {
  if !state.args.site_addresses.contains(&address) {
    return None
  }

  let mut path = Path::new(&state.args.zeronet_path).join(address).join("data").join(path);
  if path.is_dir() {
    path.push("index.html");
  }
  trace!("Data file request: {:?}", path);

  NamedFile::open(path).ok()
}

#[get("/<address>/fonts/<path..>")]
fn static_font_file(state: State<StateWrapper>, address: String, path: PathBuf) -> Option<CachedFile> {
  if !state.args.site_addresses.contains(&address) {
    return None
  }

  let mut path = Path::new(&state.args.zeronet_path).join(address).join("fonts").join(path);
  if path.is_dir() {
    path.push("index.html");
  }
  trace!("Font file request: {:?}", path);

  NamedFile::open(path).map(|nf| CachedFile::new(nf)).ok()
}
