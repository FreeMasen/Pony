pub extern crate hyper;
pub extern crate futures;
extern crate percent_encoding;
extern crate sha1;
use futures::future::Future;
use hyper::server::{Request, Response};
use hyper::Error;



pub type HyperResult = Box<Future<Item = Response, Error = Error>>;
pub type Callback = fn(Request) -> HyperResult;
pub mod pony;
pub mod pony_builder;