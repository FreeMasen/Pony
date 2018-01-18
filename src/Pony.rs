use std::collections::HashMap;
use std::fs::{File};
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use futures::future::Future;
use futures::future::ok;

use hyper::{Get, Post, Put, Delete, StatusCode, Error};
use hyper::server::{Service, Request, Response};
use hyper::header::{ContentLength};



pub type Callback = fn(Request) -> Box<Future<Item = Response, Error = Error>>;
struct Pony {
    gets: HashMap<String, Callback>,
    posts: HashMap<String, Callback>,
    puts: HashMap<String, Callback>,
    deletes: HashMap<String, Callback>,
    static_path: String,
    static_enabled: bool,
    not_found_path: String,
    custom_not_found: bool,
    known_files: Vec<&'static str>
}

impl Pony {
    fn get(&self, req: Request) -> Box<Future<Item = Response, Error = Error>> {
        match self.gets.get(req.path()) {
            Some(cb) => {
                cb(req)
            },
            None => {
                if self.static_enabled {
                    println!("using static and no routes match, checking for fallback");
                    self.static_file(req.path())
                } else {
                    self.not_found()
                }
            },
        }
    }
    
    fn static_file(&self, path: &str) -> Box<Future<Item = Response, Error = Error>> {
        let mut incoming = String::from(path);
        if incoming.ends_with('/') {
            incoming += "index.html";
        } else if !self.check_for_known_ext(&incoming) {
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
            return self.not_found()
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
            self.not_found()
        }
    }

    fn check_for_known_ext(&self, path: &str) -> bool {
        if path.ends_with("/") {
            return false;
        }
        for ext in self.known_files {
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
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("{:?}: {:?}", req.method(), req.path());
        match req.method() {
            &Get => {
                self.get(req)
            },
            &Post => {
                match self.posts.get(req.path()) {
                    Some(cb) => cb(req),
                    None => self.not_found(),
                }
            },
            &Put => {
                match self.puts.get(req.path()) {
                    Some(cb) => cb(req),
                    None => self.not_found(),
                }
            },
            &Delete => {
                match self.deletes.get(req.path()) {
                    Some(cb) => cb(req),
                    None => self.not_found(),
                }
            }
            _ => {
                self.not_found()
            }
        }
    }
}

impl Pony {
    fn not_found(&self) -> Box<Future<Item = Response, Error = Error>> {
        if self.custom_not_found {
            let path = PathBuf::from(self.not_found_path);
            let file = if let Ok(f) = File::open(path) {
                f
            } else {
                panic!(format!("Unable to find file 404 file: {:?}", self.not_found_path));
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
                Pony::default_not_found()
            }
        } else {
            Pony::default_not_found()
        }
    }

    fn default_not_found() -> Box<Future<Item = Response, Error = Error>> {
        Box::new(
            ok(
                Response::new()
                    .with_status(StatusCode::NotFound)
            )
        )
    }
}

fn default_known_extentions(path: &String) -> Vec<&str> {
        vec![
            ".html",
            ".js",
            ".css",
            ".ico",
            ".jpg",
            ".png",
            ".woff2",
            ".ttf",
            ".txt",
            ".xml",
            ".rss",
            ".svg",
            ".txt"
        ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}