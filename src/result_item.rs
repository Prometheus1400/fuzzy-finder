#[derive(Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct ResultItem {
    pub content: String,
    pub score: i64,
    pub scoring_task_id: u64,
}

impl ResultItem {
    pub fn new(content: String, score: i64, scoring_task_id: u64) -> Self {
        Self { content, score, scoring_task_id }
    }
}
