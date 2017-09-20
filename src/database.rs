use r2d2;
use r2d2_diesel::ConnectionManager;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel;
use models::{User, NewUser};
use models::{Word, NewWord};
use models::{Article, NewArticle};
use models::{ArticleWord, NewArticleWords};
use models::{UserArticle, NewUserArticle};
use models::{UserWord, NewUserWord};

// This code should be audited and reviewed
// additionally it should be optimized to not create so many Strings
pub fn create_user<'a>(conn: &PgConnection,
                       name: &'a str,
                       password: &'a str)
                       -> Result<User, diesel::result::Error> {
    use schema::users;
    use blake2::{Blake2b, Digest};
    use rand::{Rng, StdRng};

    const MAX_SALT_SIZE: usize = 16;

    let salt = {
        let mut unencoded = [0u8; MAX_SALT_SIZE];
        let mut rng = StdRng::new().expect("Could not make RNG");
        rng.fill_bytes(&mut unencoded);
        unencoded
    };
    let mut hasher = Blake2b::default();
    hasher.input(password.as_bytes());
    hasher.input(&salt);
    let password_hash = hasher.result();
    let password_hash_string = format!("{:x}", &password_hash);

    let mut salt_string = String::new();
    for byte in salt.iter() {
        salt_string += format!("{:x}", byte).as_ref();
    }

    let new_user = NewUser {
        username: name,
        password_hash: password_hash_string.as_ref(),
        salt: salt_string.as_ref(),
    };

    let user: User = diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn)?;
    //  .expect("Error adding new user");

    Ok(user)
}

pub fn create_article<'a>(conn: &PgConnection,
                          user_id: i32,
                          title: &'a str,
                          text: &'a str,
                          language_id: &'a str)
                          -> Article {
    use schema::articles;
    use schema::user_articles;

    let unique_word_ids: Vec<i32> = vec![];

    //find unique words
    let new_article = NewArticle {
        title: title,
        text: text,
        language_id: language_id,
    };

    print_sql!(diesel::insert(&new_article).into(articles::table))
;
    let article: Article = diesel::insert(&new_article)
        .into(articles::table)
        .get_result(conn)
        .expect("Error adding new article");

    let new_user_article = NewUserArticle {
        user_id: user_id,
        article_id: article.id,
    };


    diesel::insert(&new_user_article)
        .into(user_articles::table)
        .execute(conn)
        .expect("Error adding new article to user");

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
