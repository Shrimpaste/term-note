use chrono::{DateTime, Local};

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub highlighted_preview: String,
    pub updated_at: DateTime<Local>,
    pub score: f64,
}

/// 搜索查询
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    pub keyword: String,
    pub tags: Vec<String>,
    pub from_date: Option<DateTime<Local>>,
    pub to_date: Option<DateTime<Local>>,
}

impl SearchQuery {
    pub fn new(keyword: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            ..Default::default()
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}