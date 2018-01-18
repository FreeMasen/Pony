use hyper::{Get, Post, Put, Delete};
use hyper::server::{Service, NewService, Request, Response};
use hyper::header::{ContentLength};

use std::collections::HashMap;

use std::io;
use std::io::{BufReader, Read};
use std::fs::{File};
use std::path::PathBuf;

type Callback = fn(Request) -> Box<Future<Item = Response, Error = hyper::Error>>;
struct Pony {
    pub gets: HashMap<String, Callback>,
    pub posts: HashMap<String, Callback>,
    pub puts: HashMap<String, Callback>,
    pub deletes: HashMap<String, Callback>,
    pub static_path: String,
    static_enabled: bool,
}

impl Pony {
    fn static_file(&self, path: &str) -> Box<Future<Item = Response, Error = hyper::Error>> {
        let mut incoming = String::from(path);
        if incoming.ends_with('/') {
            incoming += "index.html";
        } else if !WiredForge::has_know_extention(&incoming) {
            incoming += "/index.html";
        }
        
        if self.static_path.ends_with('/') && incoming.starts_with('/') {
            incoming.remove(0);
        }
        let static_path = self.static_path.clone() + &incoming;
        let pb = PathBuf::from(static_path);
        println!("path: {:?}", &pb);
        let file = if let Ok(f) = File::open(pb) {
            println!("File opened successfully");
            f
        } else {
            println!("File failed to open");
            return WiredForge::not_found()
        };
        
        let mut reader = BufReader::new(file);
        let mut contents: Vec<u8> = vec!();
        if let Ok(_) = reader.read_to_end(&mut contents) {
            println!("File successfully read");
            Box::new(
                ok(
                    Response::new()
                        .with_body(contents)
                )
            )
        } else {
            println!("Failed to read file as bytes");
            WiredForge::not_found()
        }
    }

    fn has_know_extention(path: &String) -> bool {
        let know_files: Vec<&str> = vec![
            ".html",
            ".js",
            ".css",
            ".ico",
            ".jpg",
            ".png",
            ".woff2",
            ".ttf",
            ".txt",
            ".xml"
        ];
        for ext in know_files {
            if path.ends_with(ext) {
                return true;
            }
        }
        false
    }
}

impl Service for Pony {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("{:?}: {:?}", req.method(), req.path());
        match req.method() {
            &Get => {
                match self.gets.get(req.path()) {
                    Some(cb) => {
                        cb(req)
                    },
                    None => {
                        if self.static_enabled {
                            println!("using static and no routes match, checking for fallback");
                            self.static_file(req.path())
                        } else {
                            WiredForge::not_found()
                        }
                    },
                }
            },
            &Post => {
                match self.posts.get(req.path()) {
                    Some(cb) => cb(req),
                    None => Pony::not_found(),
                }
            },
            &Put => {
                match self.puts.get(req.path()) {
                    Some(cb) => cb(req),
                    None => Pony::not_found(),
                }
            },
            &Delete => {
                match self.deletes.get(req.path()) {
                    Some(cb) => cb(req),
                    None => Pony::not_found(),
                }
            }
            _ => {
                WiredForge::not_found()
            }
        }
    }
}

impl Pony {
    fn not_found() -> Box<Future<Item = hyper::Response, Error = hyper::Error>> {
        println!("Pony::not_found");
        let path = PathBuf::from("public/404/index.html");
        let file = if let Ok(f) = File::open(path) {
            println!("opened filed");
            f
        } else {
            println!("Failed to open file");
            return Box::new(ok(Response::new()
            .with_status(StatusCode::NotFound)));
        };
        let mut reader = BufReader::new(file);
        let mut bytes: Vec<u8> = vec!();
        if let Ok(size) = reader.read_to_end(&mut bytes) {
            println!("read file to end");
            Box::new(
                ok(
                    Response::new()
                    .with_status(StatusCode::Ok)
                    .with_header(ContentLength(size as u64))
                    .with_body(bytes)
                )
            )
        } else {
            println!("failed to read file to end");
            Box::new(
                ok(
                    Response::new()
                        .with_status(StatusCode::NotFound)
                )
            )
        }
    }
    fn new() -> WiredForge {
        WiredForge {
            gets: HashMap::new(),
            posts: HashMap::new(),
            static_path: String::new(),
            static_enabled: false,
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
