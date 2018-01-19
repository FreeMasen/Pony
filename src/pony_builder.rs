use std::collections::HashMap;
use std::path::{PathBuf};
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
    pub fn get(&mut self, path: String, cb: Callback) -> &mut PonyBuilder {
        self.gets.insert(path, cb);
        self
    }
    pub fn post(&mut self, path: String, cb: Callback) -> &mut PonyBuilder {
        self.posts.insert(path, cb);
        self
    }
    pub fn put(&mut self, path: String, cb: Callback) -> &mut PonyBuilder {
        self.puts.insert(path, cb);
        self
    }
    pub fn delete(&mut self, path: String, cb: Callback) -> &mut PonyBuilder {
        self.deletes.insert(path, cb);
        self
    }
    pub fn use_static(&mut self, path: String) -> &mut PonyBuilder {
        self.static_path = path;
        self.static_enabled = true;
        self
    }
    pub fn use_not_found(&mut self, path: &str) -> &mut PonyBuilder {
        let as_buf = PathBuf::from(&path);
        if !as_buf.exists() {
            panic!(format!("Not found path does not exist\n{:?}", &path));
        }
        self.not_found_path = path.to_string();
        self.custom_not_found = true;
        self
    }
}


#[cfg(test)]
#[allow(unused_variables, dead_code)]
mod tests {
    use futures::future::ok;
    use hyper::{Response, Request};
    fn res(_req: Request) -> super::super::HyperResult {
        Box::new(
            ok(
                Response::new()
            )
        )
    }
    #[test]
    #[should_panic]
    fn not_found_failed() {
        let mut pb = super::PonyBuilder::new();
        pb.use_not_found("/404.html");
    }
    #[test]
    fn not_found() {
        let mut pb = super::PonyBuilder::new();
        pb.use_not_found("/");
        assert!(pb.custom_not_found, true);
        assert!(pb.not_found_path == String::from("/"), true);
    }
    #[test]
    fn config_test() {
        let mut pb = super::PonyBuilder::new();
        pb.use_static(String::from("/"))
            .use_not_found("/")
            .get(String::from("/get"), res)
            .post(String::from("/post"), res)
            .put(String::from("/put"), res)
            .delete(String::from("/delete"), res);
    }
}