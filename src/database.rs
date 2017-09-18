use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel;
use models::{User, NewUser};
use models::{Article, NewArticle};
use models::{UserData, NewUserData};

pub fn create_user<'a>(conn: &PgConnection, name: String) -> Result<User, diesel::result::Error> {
    use schema::users;
    use schema::user_data;

    let new_user = NewUser { username: name.as_ref() };

    let user: User = diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn)?;
    //  .expect("Error adding new user");

    let default_user_word_ids = vec![];
    let default_user_article_ids = vec![];
    let new_user_data = NewUserData {
        user_id: user.id,
        article_ids: default_user_article_ids.as_slice(),
        users_word_ids: default_user_word_ids.as_slice(),
    };

    diesel::insert(&new_user_data)
        .into(user_data::table)
        .execute(conn)?;
    //.expect("Error adding new user data");

    Ok(user)
}

pub fn create_article<'a>(conn: &PgConnection,
                          user_id: i32,
                          title: String,
                          text: String,
                          language_name: String)
                          -> Article {
    use schema::articles;
    use schema::user_data;

    let unique_word_ids: Vec<i32> = vec![];

    //find unique words
    let new_article = NewArticle {
        title: title.as_ref(),
        text: text.as_ref(),
        language_name: language_name.as_ref(),
        unique_word_ids: unique_word_ids.as_slice(),
    };

    print_sql!(diesel::insert(&new_article).into(articles::table))
;
    let article: Article = diesel::insert(&new_article)
        .into(articles::table)
        .get_result(conn)
        .expect("Error adding new article");

    let mut user_data: UserData = user_data::table
        .find(user_id)
        .get_result(conn)
        .expect("Could not find user_data for the given user");
    user_data.article_ids.push(article.id);

    diesel::update(user_data::table.filter(user_data::user_id.eq(user_id)))
        .set(user_data::article_ids.eq(user_data.article_ids))
        .execute(conn)
        .expect("Could not update user data with new article");

    article
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
