use std::ffi::OsStr;
use std::{path::PathBuf, sync::mpsc::Receiver};
use notify::{DebouncedEvent, INotifyWatcher, Watcher, RecursiveMode};
use log::*;
use crate::zero_site::SiteData;
use std::sync::{Arc, Mutex};
use std::fs::{self, File};
use crate::args::Args;
use std::collections::HashMap;

pub fn watch(args: Args, mut watcher: INotifyWatcher, receiver: Receiver<DebouncedEvent>, sites: HashMap<String, Arc<Mutex<SiteData>>>) {
  info!("Listening for file changes...");
  let mut file_watcher = FileWatcher {
    notify_watcher: watcher,
    sites,
    args,
  };

  file_watcher.load_sites();

  loop {
    match receiver.recv() {
      Ok(DebouncedEvent::Remove(path)) | Ok(DebouncedEvent::Write(path)) => file_watcher.load_file(path),
      Ok(event) => info!("{:?}", event),
      Err(e) => error!("{:?}", e),
    }
  }
}

struct FileWatcher {
  notify_watcher: INotifyWatcher,
  sites: HashMap<String, Arc<Mutex<SiteData>>>,
  args: Args,
}

impl FileWatcher {
  fn load_file(&mut self, path: PathBuf) {
    for (key, site) in self.sites.iter() {
      if path.components().any(|c| format!("{:?}", c) == *key) {
        let mut site = site.lock().unwrap();
        match path.file_name() == Some(OsStr::new("content.json")) {
          true => site.load_content(format!("{:?}", path)),
          false => site.load_data(format!("{:?}", path)),
        }
      }
    }
  }

  fn load_sites(&mut self) {
    for (address, site) in self.sites.iter() {
      let mut site = site.lock().unwrap();
      let path = format!("{}/{}/content.json", self.args.zeronet_path, address);
      site.load_content(path);
      let path = format!("{}/{}/data/data.json", self.args.zeronet_path, address);
      site.load_data(path);
    }
  }

  // fn load_content(&mut self, path: PathBuf) {
  //   // let path = format!("{}/{}/content.json", self.args.zeronet_path, self.args.site_addresses[0]);
  //   info!("Reading from {}", path);
  //   let contents = fs::read_to_string(path.clone()).unwrap();

  //   let mut state = self.shared_state.lock().unwrap();
  //   state.content = serde_json::from_str(&contents).unwrap();

  //   info!("Watching {}", path);
  //   self.notify_watcher.watch(path, RecursiveMode::NonRecursive);
  // }

  // fn load_data(&mut self, path: PathBuf) {
  //   // let path = format!("{}/{}/data/data.json", self.args.zeronet_path, self.args.site_addresses[0]);
  //   info!("Reading from {:?}", path);
  //   let contents = fs::read_to_string(path.clone()).unwrap();

  //   let mut state = self.shared_state.lock().unwrap();
  //   state.data = serde_json::from_str(&contents).unwrap();

  //   info!("Watching {:?}", path);
  //   self.notify_watcher.watch(path, RecursiveMode::NonRecursive);
  // }
}

fn update_state(watcher: &mut INotifyWatcher, shared_state: Arc<Mutex<SiteData>>, path: PathBuf) {
  let contents = fs::read_to_string(path.clone()).unwrap();

  watcher.watch(path, RecursiveMode::NonRecursive);

  let state = shared_state.lock().unwrap();
  debug!("Content: {:?}", state.content.modified);
  debug!("Data: {:?}", state.data.modified);
}
