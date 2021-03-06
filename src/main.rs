#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_codegen;
extern crate blake2;
extern crate rand;


use self::diesel::prelude::*;

use std::io;

use rocket::response::NamedFile;
use rocket::http::{Cookies, Cookie};
use rocket::Request;
use rocket::response::Redirect;
use rocket::response::Flash;
use rocket::request;
use rocket::request::{FromRequest, FlashMessage, Form, LenientForm};
use rocket::outcome::IntoOutcome;
//use rocket::response::content::Json;
use rocket_contrib::{Json, Template};

pub mod models;
pub mod schema;
pub mod database;
pub mod dbconnectionguard;
pub mod articleinfo;

use dbconnectionguard::*;
use articleinfo::{ArticleInfo, ProcessedArticleInfo};

use database::*;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/js/<filename>")]
fn static_js(filename: String) -> io::Result<NamedFile> {
    NamedFile::open(format!("static/{}", filename))
}

#[get("/login")]
fn login_page() -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
}

#[get("/create_account")]
fn create_account_page() -> io::Result<NamedFile> {
    NamedFile::open("static/create_account.html")
}

#[get("/hello")]
fn hello(_flash: Option<FlashMessage>, mut cookies: Cookies) -> Option<String> {
    cookies
        .get_private("user_name")
        .map(|cookie| format!("Hello, {}", cookie.value()))
}

#[derive(Deserialize, FromForm)]
struct Login {
    username: String,
    password: String,
}

#[derive(Debug)]
struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}

#[post("/login", data="<login>")]
fn login(conn: DbConn, mut cookies: Cookies, login: Form<Login>) -> Flash<Redirect> {
    use schema::users;
    use models::User;
    let dbconn = conn.0;
    let muser: Result<User, _> = users::table
        .filter(users::username.eq(login.get().username.clone()))
        .limit(1)
        .get_result(&*dbconn);

    if let Ok(user) = muser {
        cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
        cookies.add_private(Cookie::new("user_name", format!("{}", user.username)));
        return Flash::success(Redirect::to("/articles"), "Successfully logged in.");
    } else {
        return Flash::error(Redirect::to("/login"), "Invalid username or password");
    }
}

#[post("/create_account", data="<login>")]
fn create_account(conn: DbConn,
                  mut cookies: Cookies,
                  login: LenientForm<Login>)
                  -> Flash<Redirect> {
    use schema::users;
    use models::User;
    let dbconn = conn.0;
    let login_user = login.into_inner();
    let muser: Result<User, _> = create_user(&*dbconn,
                                             login_user.username.as_ref(),
                                             login_user.password.as_ref());

    if let Ok(user) = muser {
        cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
        cookies.add_private(Cookie::new("user_name", format!("{}", user.username)));
        return Flash::success(Redirect::to("/articles"), "Successfully logged in.");
    } else {
        return Flash::error(Redirect::to("/login"), "Invalid username or password");
    }
}


#[get("/article/<article_id>")]
fn load_article_page(mut cookies: Cookies, conn: DbConn, article_id: i32) -> Option<Template> {
    use serde_json::value::{Value as Json, Map};
    cookies
        .get_private("user_id")
        .and_then(|value| {
            value
                .value()
                .to_string()
                .parse::<i32>()
                .ok()
                .and_then(|user_id| get_article(&*conn, user_id, article_id).ok())
                .map(|article| {
                         let mut con_map = Map::new();
                         con_map.insert("article".to_owned(), json!(article));
                         Template::render("article", con_map)
                     })
        })
}


#[allow(resolve_trait_on_defaulted_unit)]
#[get("/articles")]
fn articles_hub(mut cookies: Cookies, conn: DbConn) -> Option<Template> {
    use serde_json::value::{Value as Json, Map};

    cookies
        .get_private("user_id")
        .and_then(|value| {
            value
                .value()
                .to_string()
                .parse::<i32>()
                .ok()
                .and_then(|user_id| get_articles(&*conn, user_id, 10).ok())
                .map(|articles| {
                         let mut con_map = Map::new();
                         con_map.insert("articles".to_owned(), json!(articles));
                         Template::render("articles", con_map)
                     })
        })
}

#[get("/article_content/<article_id>")]
fn load_article(mut cookies: Cookies,
                conn: DbConn,
                article_id: i32)
                -> Result<Json<ProcessedArticleInfo>, Redirect> {
    use schema::users;
    use schema::articles;
    //    use schema::users::dsl;
    //   use schema::articles::dsl;
    use models::{Article, User};

    cookies
        .get_private("user_id")
        .map(|value| {
            let user_id: i32 = value.value().to_string().parse::<i32>().unwrap();
            let user: User = users::table
                .find(user_id)
                .first(&*conn)
                .expect("Could not find user when looking for article");

            unimplemented!("Replace userdata below");

            /*let user_data: UserData =
                UserData::belonging_to(&user)
                    .first(&*conn)
                    .expect("Could not find user data when looking for article");

            let article_id_iter = user_data.article_ids.iter().take(10);

            let article: Article = articles::table
                .find(article_id)
                .limit(1)
                .first(&*conn)
                .expect("Could not find any articles");

            /*.as_query()
                .find(article_id)
                .get_result(&*conn)
                .expect("could not find article");*/

            return Ok(Json(article.process(|text| {
                                               text.split_whitespace()
                                                   .map(|word| word.to_string())
                                                   .collect()
                                           },
                                           user_id,
                                           conn)));*/
        });

    Err(Redirect::to("/login"))
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("name"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[get("/new_article")]
fn new_article_page() -> io::Result<NamedFile> {
    NamedFile::open("static/NewArticle.html")
}

#[post("/new_article", data = "<info>")]
fn new_article(mut cookies: Cookies, info: Json<ArticleInfo>, conn: DbConn) -> Option<Redirect> {
    cookies
        .get_private("user_id")
        .map(|value| {
            //find this with the cookie
            let user_id: i32 = value.value().to_string().parse::<i32>().unwrap();
            let article = create_article(&*conn,
                                         user_id,
                                         info.title.as_ref(),
                                         info.text.as_ref(),
                                         info.language_name.as_ref());
            return Some(Redirect::to(format!("/article/{}", article.id).as_ref()));
        });
    None
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("404: file not found: request: {:?}", req)
}

fn main() {
    let connection = create_db_pool();

    rocket::ignite()
        .mount("/",
               routes![index,
                       login,
                       hello,
                       new_article,
                       new_article_page,
                       load_article,
                       load_article_page,
                       articles_hub,
                       login_page,
                       create_account,
                       create_account_page,
                       logout,
                       static_js])
        .manage(connection)
        .attach(Template::fairing())
        .launch();
}
