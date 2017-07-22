#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::http::{Cookies, Cookie};
use rocket::Data;
use rocket::Request;
use rocket::response::Redirect;

#[derive(FromForm)]
struct Message {
    message: String,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(file)).ok()
}

#[get("/hello")]
fn hello(cookies: Cookies) -> Option<String> {
    cookies.get("name").map(|value| format!("Hello, {}", value))
}

#[post("/test", data = "<message>")]
fn submit_name(mut cookies: Cookies, message: Form<Message>) -> Redirect {
    cookies.add(Cookie::new("message", message.into_inner().message));
    Redirect::to("/hello")
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("404: file not found")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, submit_name, files])
        .launch();
}
