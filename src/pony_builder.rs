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
    known_extensions: Vec<String>,
    custom_know_extensions: bool,
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
            known_extensions: Vec::new(),
            custom_know_extensions: false,
        }
    }

    pub fn done(&mut self) -> Pony {
        let known_files = if !self.custom_know_extensions {
            PonyBuilder::default_known_extentions()
        } else {
            self.known_extensions.clone()
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
    pub fn get(&mut self, path: &str, cb: Callback) -> &mut Self {
        self.gets.insert(path.to_string(), cb);
        self
    }
    pub fn post(&mut self, path: &str, cb: Callback) -> &mut Self {
        self.posts.insert(path.to_string(), cb);
        self
    }
    pub fn put(&mut self, path: &str, cb: Callback) -> &mut Self {
        self.puts.insert(path.to_string(), cb);
        self
    }
    pub fn delete(&mut self, path: &str, cb: Callback) -> &mut Self {
        self.deletes.insert(path.to_string(), cb);
        self
    }
    pub fn use_static(&mut self, path: &str) -> &mut PonyBuilder {
        let as_buf = PathBuf::from(&path);
        if !as_buf.exists() {
            panic!(format!("Static path does not exist\n{:?}", &path));
        }
        if as_buf.is_file() {
            panic!(format!("Static path must be a directory\n{:?}", &path));
        }
        self.static_path = path.to_string();
        self.static_enabled = true;
        self
    }
    pub fn use_not_found(&mut self, path: &str) -> &mut Self {
        let as_buf = PathBuf::from(&path);
        if !as_buf.exists() ||
        !as_buf.is_file() {
            panic!(format!("Not found path does not exist\n{:?}", &path));
        }
        if as_buf.is_dir() {
            panic!(format!("Not found path is a directory"))
        }
        self.not_found_path = path.to_string();
        self.custom_not_found = true;
        self
    }
    pub fn set_know_extensions(&mut self, list: &[&str]) -> &mut Self {
        self.known_extensions = list.into_iter().map(|e| e.to_string()).collect();
        self.custom_know_extensions = true;
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
    #[should_panic]
    fn not_found_dir() {
        let mut pb = super::PonyBuilder::new();
        pb.use_not_found("/");
    }
    #[test]
    fn not_found() {
        let path = "examples/public/index.html";
        let mut pb = super::PonyBuilder::new();
        pb.use_not_found(&path);
        assert!(pb.custom_not_found, "pb.custom_not_found was not set to true");
        assert!(pb.not_found_path == path.to_string(), "pb.not_found_path did not match");
    }
    #[test]
    #[should_panic]
    fn static_test_failed() {
        let mut pb = super::PonyBuilder::new();
        pb.use_static("junk/");
    }
    #[test]
    #[should_panic]
    fn static_test_file() {
        let mut pb = super::PonyBuilder::new();
        pb.use_static("examples/public/index.html");
    }
    #[test]
    fn static_test() {
        let mut pb = super::PonyBuilder::new();
        let path = "examples/public/";
        pb.use_static(&path);
        assert!(pb.static_enabled, "pb.static_enabled is not set to true");
        assert!(pb.static_path == path, "pb.static_path does not match");
    }
    #[test]
    fn config_test() {
        let mut pb = super::PonyBuilder::new();
        pb.use_static("examples/public/")
            .use_not_found("examples/public/index.html")
            .get("/get", res)
            .post("/post", res)
            .put("/put", res)
            .delete("/delete", res);
    }
}