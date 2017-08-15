#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_codegen;


use self::diesel::prelude::*;

use std::io;
use std::path::{Path, PathBuf};

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::http::{Cookies, Cookie};
use rocket::Request;
use rocket::response::Redirect;
//use rocket::response::content::Json;
use rocket::response::Flash;
use rocket_contrib::Json;

pub mod models;
pub mod schema;
pub mod database;
pub mod dbconnectionguard;
pub mod articleinfo;

use models::User;
use dbconnectionguard::*;
use articleinfo::ArticleInfo;

use database::*;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/login")]
fn login_page() -> io::Result<NamedFile> {
    NamedFile::open("static/a.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static").join(file)).ok()
}

#[get("/hello")]
fn hello(mut cookies: Cookies) -> Option<String> {
    cookies
        .get_private("name")
        .map(|value| format!("Hello, {}", value.value()))
}

#[post("/test", data = "<message>")]
fn submit_name(mut cookies: Cookies, message: String) -> Redirect {
    cookies.add_private(Cookie::new("name", message));
    Redirect::to("/hello")
}

#[post("/login", format="text/plain", data="<name>")]
fn login(mut cookies: Cookies, name: String) -> Redirect {
    cookies.add_private(Cookie::new("name", name.clone()));
    Redirect::to("/hello")
}

/*#[post("/register", format="text/plain", data="<name>")]
fn register(mut cookies: Cookies, conn: mut DbConn, name: String) -> Redirect {
    conn.
    cookies.add_private(Cookie::new("name", name.clone()));
    Redirect::to("/hello")
}*/



#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("name"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[post("/new_article", data = "<info>")]
fn new_article(mut cookies: Cookies, info: Json<ArticleInfo>, conn: DbConn) -> String {
    use schema::users;
    use schema::users::dsl;
    cookies
        .get_private("id")
        .map(|value| {
                 //find this with the cookie
                 let user_id: i32 = value.value().to_string().parse::<i32>().unwrap();
                 create_article(&*conn, user_id, info.title.clone(), info.text.clone());
                 return format!("Success");
             });

    format!("Failure")
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("404: file not found: request: {:?}", req)
}

fn main() {
    use schema::users;

    let connection = create_db_pool; //establish_connection();

    use schema::users::dsl;

    /*    for id in 1..4 {
        let post = diesel::update(dsl::posts.find(id))
            .set(dsl::published.eq(true))
            .get_result::<Post>(&connection)
            .expect(&format!("unable to find post {}", id));
        println!("Published post {}", id);
    }


    let results = posts::table
        .filter(posts::published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-------------\n");
        println!("{}", post.body);
    }*/

    rocket::ignite()
        .mount("/",
               routes![index,
                       login,
                       hello,
                       new_article,
                       submit_name,
                       login_page,
                       logout,
                       files])
        .manage(connection)
        .launch();
}
