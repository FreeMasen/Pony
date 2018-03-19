extern crate hyper;
extern crate pony;
extern crate futures;
use hyper::server::{Http, NewService};
use pony::pony_builder::PonyBuilder;

mod routes;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut pb = PonyBuilder::new();
    pb.get("/get", routes::get);
    pb.put("/put", routes::put);
    pb.post("/post", routes::post);
    pb.delete("/delete", routes::delete);
    let handler = Http::new().bind(&addr, move || pb.new_service()).unwrap();
    handler.run().unwrap();
}