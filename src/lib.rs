extern crate hyper;
extern crate futures;

use futures::future::Future;
use hyper::server::{Request, Response};
use hyper::Error;

type HyperResult = Box<Future<Item = Response, Error = Error>>;
pub type Callback = fn(Request) -> HyperResult;
pub mod pony;
pub mod pony_builder;