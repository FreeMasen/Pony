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
    pub known_extensions: HashSet<String>,
    pub static_logging: bool,
    pub use_gzip: bool
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
                    if self.static_logging {
                        println!("GET: {:?}", req.path());
                    }
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
        let contents = if self.use_gzip {
            match Self::read_file(PathBuf::from(static_path.clone() + ".gz")) {
                Ok(content) => Ok(content),
                Err(_) => Self::read_file(PathBuf::from(static_path))
            }
        } else {
            Self::read_file(PathBuf::from(static_path))
        };

        match contents {
            Ok(c) => {
                println!("{:?}", path);
                Box::new(
                    ok(
                        Response::new()
                            .with_body(c)
                    )
                )
            },
            Err(_) => self.not_found()
        }
    }
    //attempt to read a file
    fn read_file(path: PathBuf) -> Result<Vec<u8>, String> {
        let file = if let Ok(f) = File::open(path) {
            f
        } else {
            return Err(String::from("File not found"))
        };
        let mut reader = BufReader::new(file);
        let mut contents: Vec<u8> = vec!();
        if let Ok(_) = reader.read_to_end(&mut contents) {
            Ok(contents)
        } else {
            return Err(String::from("File unreadable"))
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
    // use super::*;
    // use super::super::pony_builder::PonyBuilder;
    // use super::super::HyperResult;
    // use hyper::server::Request;
    // use hyper::{Method, Uri};
    // use std::str::FromStr;
    // use futures::future::ok;
    // use std::boxed::Box;
    // use std::ops::Deref;
    // #[test]
    // fn route_test() {
    //     let p = PonyBuilder::new()
    //                         .get("/get", route)
    //                         .post("/post", route)
    //                         .put("/put", route)
    //                         .delete("/delete", route)
    //                         .done();
    //     let get = Request::new(Method::Get, Uri::from_str("/get").unwrap());
    //     let g = p.call(get).then(|r| {
    //         r
    //     });

    //     // let put = Request::new(Method::Put, Uri::from_str("/put").body());
    //     // assert!(p.call(put).body() == response(&Method::Put).body());
    //     // let post = Request::new(Method::Post, Uri::from_str("/post"));
    //     // assert!(p.call(post).body() == response(&Method::Post).body());
    //     // let delete = Request::new(Method::Delete, Uri::from_str("/delete"));
    //     // assert!(p.call(delete).body() == response(&Method::Delete).body());
    // }

    // fn route(req: Request) -> HyperResult {
    //     response(req.method())
    // }

    // fn response(method: &Method) -> HyperResult {
    //     let body = match method {
    //         &Method::Get => "GET",
    //         &Method::Put => "PUT",
    //         &Method::Post => "POST",
    //         &Method::Delete => "DELETE",
    //         _ => "UNKNOWN"
    //     };
    //     Box::new(ok(Response::new()
    //                 .with_body(body)))
    // }

    // #[test]
    // fn static_test() {

    // }
}