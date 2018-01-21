use std::collections::{HashMap,HashSet};
use std::fs::{File};
use std::io::{BufReader, Read};
use std::path::PathBuf;

use futures::future::ok;

use hyper::{Get, Post, Put, Delete, StatusCode, Error};
use hyper::server::{Service, Request, Response};
use hyper::header::{ContentLength};

use super::Callback;
///A set of hyper http settings
pub struct Pony {
    pub gets: HashMap<String, Callback>,
    pub posts: HashMap<String, Callback>,
    pub puts: HashMap<String, Callback>,
    pub deletes: HashMap<String, Callback>,
    pub static_path: String,
    pub static_enabled: bool,
    pub not_found_path: String,
    pub custom_not_found: bool,
    pub known_extensions: HashSet<String>
}

impl Pony {
    ///Try to perform a get request, if path is not found in
    /// this instance's gets HashMap and static files are enabled
    /// it will attempt to find a static file.
    /// note this will attempt to find index.html if no file extention
    /// exists on request
    fn get(&self, req: Request) -> super::HyperResult {
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
    ///Fallback when any get request's path doesn't exist
    /// in this instance's gets HashMap
    fn static_file(&self, path: &str) -> super::HyperResult {
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
    ///Check for a path's extention to be in our list of
    /// known extensions
    fn check_for_known_ext(&self, path: &str) -> bool {
        if path.ends_with("/") {
            return false;
        }
        let ext = path.split('.').last().expect("failed to get last item in path");
        self.known_extensions.contains(ext)
    }
}

impl Service for Pony {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = super::HyperResult;
    ///This is used by hyper to respond to any requests
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
    ///This will return the default 404 text or
    /// a custom 404 .html file if one was provided
    fn not_found(&self) -> super::HyperResult {
        if self.custom_not_found {
            let path = PathBuf::from(&self.not_found_path);
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
    ///The default 404
    fn default_not_found() -> super::HyperResult {
        Box::new(
            ok(
                Response::new()
                    .with_status(StatusCode::NotFound)
            )
        )
    }
}

#[cfg(test)]
mod tests {
    //TODO: add tests here?
}