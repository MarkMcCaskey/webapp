use super::schema::users;
use super::schema::articles;
use super::schema::words;
use super::schema::languages;
use super::schema::user_encountered_words;
use super::schema::user_data;
use articleinfo::ProcessedArticleInfo;
use dbconnectionguard::DbConn;

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Identifiable)]
pub struct Word {
    pub id: i32,
    pub text_representation: String,
    pub language_specific_word_data: Option<String>,
}

#[derive(Queryable, Identifiable)]
pub struct Language {
    pub id: String,
    pub word_properties_json: Option<String>,
}


#[derive(Queryable, Associations)]
#[belongs_to(Word)]
pub struct UserEncounteredWord {
    pub word_id: i32,
    pub definition: String,
    pub user_knowledge_level: i32,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Language, foreign_key="language_name")]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub language_name: String,
    pub unique_word_ids: Option<Vec<i32>>,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[primary_key(user_id)]
#[table_name = "user_data"]
pub struct UserData {
    pub user_id: i32,
    pub article_ids: Vec<i32>,
    pub users_word_ids: Vec<i32>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
}

#[derive(Insertable)]
#[table_name="words"]
pub struct NewWord<'a> {
    pub text_representation: &'a str,
    pub language_specific_word_data: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="languages"]
pub struct NewLanguage<'a> {
    pub id: &'a str,
    pub word_properties_json: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="user_encountered_words"]
pub struct NewUserEncounteredWord<'a> {
    pub word_id: i32,
    pub definition: &'a str,
    pub user_knowledge_level: i32,
}

#[derive(Insertable)]
#[table_name="articles"]
pub struct NewArticle<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub language_name: &'a str,
    pub unique_word_ids: &'a [i32],
}

#[derive(Insertable)]
#[table_name="user_data"]
pub struct NewUserData<'a> {
    pub user_id: i32,
    pub article_ids: &'a [i32],
    pub users_word_ids: &'a [i32],
}


impl Article {
    pub fn process<F>(&self,
                      morphological_separator: F,
                      user_id: i32,
                      conn: DbConn)
                      -> ProcessedArticleInfo
        where F: Fn(String) -> Vec<String>
    {
        use schema::user_data;
        use schema::words;
        use diesel::FindDsl;
        use diesel::LoadDsl;

        let user_data: UserData = user_data::table
            .find(user_id)
            .get_result(&*conn)
            .expect("Could not find user while processing article");

        // very slow, fix
        let unknown_word_ids = if let Some(ref uwis) = self.unique_word_ids {
            uwis.iter()
                .filter(|x| !user_data.users_word_ids.contains(x))
                .map(|&x| x)
                .collect()
        } else {
            vec![]
        };

        let unknown_words: Vec<Word> = unknown_word_ids
            .iter()
            .map(|wid| {
                     words::table
                         .find(wid)
                         .get_result(&*conn)
                         .expect("Error: Article's unique word list")
                 })
            .collect();

        let unknown_words_text: Vec<String> = unknown_words
            .iter()
            .map(|w| w.text_representation.clone())
            .collect();

        /*let unknown_word_ids_in_article: Vec<i32> = unknown_word_ids
            .iter()
            .filter(|ref wid| {
                        words::table
                            .find(words::id.eq(&wid))
                            .execute(&*conn)
                            .is_ok()
                    })
            .collect();*/

        /*        let unknown_words: Vec<String> = unknown_word_ids_in_article
            .iter()
            .map(|&wid| {
                     let word: Word =
                         words::table
                             .find(wid)
                             .get_result(&*conn)
                             .expect("Word disappeared while finding unique in article");
                     word.text_representation
                 })
            .collect();*/

        //let unknown_words = morphological_separator(self.text.clone());

        ProcessedArticleInfo {
            title: self.title.clone(),
            text: self.text.clone(),
            unknown_words: unknown_words_text,
        }
    }
}
