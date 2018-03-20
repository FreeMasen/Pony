use hyper::{Method, StatusCode};
use hyper::server::{Response, Request};
use pony::HyperResult;
use futures::future::ok;
use std::boxed::Box;
use hyper::header::ContentLength;

pub fn get(req: Request) -> HyperResult {
    method_route(req)
}

pub fn post(req: Request) -> HyperResult {
    method_route(req)
}

pub fn put(req: Request) -> HyperResult {
    method_route(req)
}

pub fn delete(req: Request) -> HyperResult {
    method_route(req)
}

fn method_route(req: Request) -> HyperResult {
    let body = match req.method() {
        &Method::Get => "GET",
        &Method::Put => "PUT",
        &Method::Post => "POST",
        &Method::Delete => "DELETE",
        _ => "UNKNOWN"
    };
     Box::new(
        ok(
            Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentLength(body.len() as u64))
            .with_body(body)
        )
    )
}