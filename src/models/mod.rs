use chrono::{DateTime, Local};

/// 笔记实体
#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Note {
    /// 创建新笔记
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        let now = Local::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            content: content.into(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新笔记内容
    pub fn update(&mut self, title: Option<String>, content: Option<String>) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(content) = content {
            self.content = content;
        }
        self.updated_at = Local::now();
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// 获取预览文本（前 N 个字符）
    pub fn preview(&self, max_len: usize) -> String {
        let content = self.content.lines().next().unwrap_or("")
            .replace('\n', " ")
            .replace('\t', " ");

        if content.len() > max_len {
            format!("{}...", &content[..max_len])
        } else {
            content
        }
    }

    /// 获取格式化日期
    pub fn format_date(&self) -> String {
        self.updated_at.format("%Y-%m-%d %H:%M").to_string()
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new("新笔记", "")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewState {
    List,
    View,
    Edit,
    Search,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditMode {
    Normal,
    Insert,
}