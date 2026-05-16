#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: i64,
    pub url: String,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
}
