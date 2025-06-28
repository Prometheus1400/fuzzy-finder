#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct ResultItem {
    pub content: String,
    pub score: i64,
}

impl ResultItem {
    pub fn new(content: String, score: i64) -> Self {
        Self { content, score }
    }
}
