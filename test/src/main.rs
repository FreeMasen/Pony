extern crate pony;
use pony::hyper::server::{NewService, Http, Request, Response};
use pony::hyper::Method;
use pony::futures::future::ok;
use pony::pony_builder::PonyBuilder;
fn main() {
    
    let mut pb = PonyBuilder::new();
    pb.get("/get", endpoint)
        .post("/post", endpoint)
        .put("/put", endpoint)
        .delete("/delete", endpoint)
        .use_static("./www")
        .use_static_gzip();
    let addr = "127.0.0.1:7878".parse().unwrap();
    let h = Http::new().bind(&addr, move || pb.new_service()).expect("Unable to start server");
    let _ = h.run();
    println!("server closing");
}


fn endpoint(req: Request) -> pony::HyperResult {
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
                .with_body(body)
        )
    )
}