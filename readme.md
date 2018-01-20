# Pony

An Express-like wrapper around [Hyper](https://hyper.rs/)

Very little has been done to abstract Hyper away from the end user. The idea being that providing a simple api for declaring rest endpoints makes developing easier.

### Example

```rust
extern crate pony;
extern crate futures;
extern crate hyper;

use futures::future::ok;
use pony::PonyBuilder;
use hyper::server::Response;
use hyper::header::ContentLength;

fn main() {
    let mut pb = PonyBuilder.new();
    pb.get("/get", )
}

fn hello_world() {
    let hw = "hello world"
    Box::new(
        ok(
            Response::new()
            .with_header(ContentLength(hw.len()))
            .with_body(&hw)
        )
    )
}
```