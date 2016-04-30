#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter};

mod controller;

fn main() {
    let mut server = Nickel::new();

    server.get("/health_check", controller::status::health_check);
    server.get("/", controller::status::hello_world);
    server.listen("127.0.0.1:6767");
}