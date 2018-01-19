use std::collections::HashMap;

// use hyper::server::NewService;

use super::pony::{Pony};
use super::Callback;

pub struct PonyBuilder {
    gets: HashMap<String, Callback>,
    posts: HashMap<String, Callback>,
    puts: HashMap<String, Callback>,
    deletes: HashMap<String, Callback>,
    static_path: String,
    static_enabled: bool,
    not_found_path: String,
    custom_not_found: bool,
    known_files: Vec<String>
}

impl PonyBuilder {
    pub fn new() -> PonyBuilder {
        PonyBuilder {
            gets: HashMap::new(),
            posts: HashMap::new(),
            puts: HashMap::new(),
            deletes: HashMap::new(),
            static_path: String::new(),
            static_enabled: false,
            not_found_path: String::new(),
            custom_not_found: false,
            known_files: Vec::new(),
        }
    }

    pub fn done(&mut self) -> Pony {
        let known_files = if self.known_files.len() < 1 {
            PonyBuilder::default_known_extentions()
        } else {
            self.known_files.clone()
        };
        Pony {
            gets: self.gets.clone(),
            posts: self.posts.clone(),
            puts: self.puts.clone(),
            deletes: self.deletes.clone(),
            static_path: self.static_path.clone(),
            static_enabled: self.static_enabled == true,
            not_found_path: self.not_found_path.clone(),
            custom_not_found: self.custom_not_found == true,
            known_files: known_files,
        }
    }

    fn default_known_extentions() -> Vec<String> {
        vec![
            String::from(".html"),
            String::from(".js"),
            String::from(".css"),
            String::from(".ico"),
            String::from(".jpg"),
            String::from(".png"),
            String::from(".woff2"),
            String::from(".ttf"),
            String::from(".txt"),
            String::from(".xml"),
            String::from(".rss"),
            String::from(".svg"),
            String::from(".txt"),
        ]
    }

}

impl PonyBuilder {
    pub fn get(&mut self, path: String, cb: Callback) -> &Self {
        self
    }
    pub fn post(&mut self, path:String, cb: Callback) -> &Self {
        self
    }
    pub fn put(&mut self, path: String, cb: Callback) -> &Self {
        self
    }
    pub fn delete(&mut self, path: String, cb: Callback) -> &Self {
        self
    }
    pub fn use_static(&mut self, path: String) -> &Self {
        self
    }
    pub fn use_not_found(&mut self, path: String) -> &Self {
        self
    }
}
