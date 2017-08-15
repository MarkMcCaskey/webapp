use super::schema::users;
use super::schema::articles;

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Article {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}

#[derive(Insertable)]
#[table_name="articles"]
pub struct NewArticle<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub text: &'a str,
}

/*#[derive(Insertable)]
#[table_name="usersarticles"]
pub struct NewUsersArticle {
    pub userid: i32,
    pub articleid: i32,
}*/
