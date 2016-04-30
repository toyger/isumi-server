extern crate nickel;
extern crate rustc_serialize;
extern crate hyper;

use nickel::{Request, Response, MiddlewareResult, MediaType};
use nickel::status::StatusCode;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct JsonResponse<T> {
    pub data: T,
    pub meta: Meta
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Meta {
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Health {
    pub status: bool
}

pub fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Hello World")
}    

pub fn health_check<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let value = JsonResponse {
        data: Health { status: true },
        meta: Meta {}
    };

    let enc = json::encode(&value).unwrap();
    res.set(MediaType::Json);
    res.set(StatusCode::Ok);
    res.send(enc)
}    