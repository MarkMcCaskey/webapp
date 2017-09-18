#[derive(Deserialize)]
pub struct ArticleInfo {
    pub title: String,
    pub text: String,
    pub language_name: String,
}

#[derive(Serialize)]
pub struct ProcessedArticleInfo {
    pub title: String,
    pub text: String,
    pub unknown_words: Vec<String>,
}
