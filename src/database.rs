use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel;
use models::{User, NewUser};
use models::{Article, NewArticle};

pub fn create_user<'a>(conn: &PgConnection, name: String) -> User {
    use schema::users;

    let new_user = NewUser { username: name.as_ref() };

    diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn)
        .expect("Error adding new user")
}

pub fn create_article<'a>(conn: &PgConnection,
                          user_id: i32,
                          title: String,
                          text: String)
                          -> Article {
    use schema::articles;

    let new_article = NewArticle {
        user_id: user_id,
        title: title.as_ref(),
        text: text.as_ref(),
    };

    diesel::insert(&new_article)
        .into(articles::table)
        .get_result(conn)
        .expect("Error adding new article")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(config, manager).expect("Failed to create pool.")
}
