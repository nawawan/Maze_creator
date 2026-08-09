use std::fmt;

use chrono::{Months, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BlogStatus {
    Draft,
    Published,
}

impl fmt::Display for BlogStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlogStatus::Draft => write!(f, "DRAFT"),
            BlogStatus::Published => write!(f, "PUBLISHED"),
        }
    }
}

impl From<String> for BlogStatus {
    fn from(status: String) -> Self {
        match status.as_str() {
            "DRAFT" => BlogStatus::Draft,
            "PUBLISHED" => BlogStatus::Published,
            _ => panic!("Unknown blog status: {}", status),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BlogTag {
    Travel,
    Retrospective,
    Diary,
    Tech,
}

impl fmt::Display for BlogTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlogTag::Travel => write!(f, "TRAVEL"),
            BlogTag::Retrospective => write!(f, "RETROSPECTIVE"),
            BlogTag::Diary => write!(f, "DIARY"),
            BlogTag::Tech => write!(f, "TECH"),
        }
    }
}

impl TryFrom<String> for BlogTag {
    type Error = String;

    fn try_from(tag: String) -> Result<Self, Self::Error> {
        match tag.as_str() {
            "TRAVEL" => Ok(BlogTag::Travel),
            "RETROSPECTIVE" => Ok(BlogTag::Retrospective),
            "DIARY" => Ok(BlogTag::Diary),
            "TECH" => Ok(BlogTag::Tech),
            _ => Err(format!("Unknown blog tag: {}", tag)),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Blog {
    pub id: Uuid,
    pub title: String,
    pub content_key: String,
    pub slug: String,
    #[sqlx(try_from = "String")]
    pub status: BlogStatus,
    pub published_at: Option<NaiveDateTime>,
    pub tag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BlogRequest {
    pub id: String,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: String,
    pub status: Option<BlogStatus>,
    pub tag: Option<String>,
}

pub struct BlogFilter {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub order_by: Option<String>,
    pub order_desc: Option<bool>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub status: Option<BlogStatus>,
    pub tag: Option<String>,
}

impl Blog {
    pub fn default_title() -> String {
        return "Untitled".to_string();
    }
}

impl BlogFilter {
    pub fn new(
        year: Option<&String>,
        month: Option<&String>,
        status: Option<BlogStatus>,
        tag: Option<&String>,
    ) -> Self {
        let (start, end) = converter_string_to_datetime(year, month);
        Self {
            start,
            end,
            tag: tag.cloned(),
            order_by: None,
            limit: None,
            offset: None,
            order_desc: None,
            status,
        }
    }

    pub fn apply(&self, query: &mut sqlx::QueryBuilder<sqlx::Postgres>) {
        if let Some(start) = self.start {
            query.push(" AND created_at >= ").push_bind(start);
        }
        if let Some(end) = self.end {
            query.push(" AND created_at <= ").push_bind(end);
        }
        if let Some(status) = &self.status {
            query.push(" AND status = ").push_bind(status.to_string());
        }
        if let Some(tag) = &self.tag {
            query.push(" AND tag = ").push_bind(tag.clone());
        }
        if let Some(order_by) = &self.order_by {
            query.push(" ORDER BY ").push(order_by);
            if self.order_desc == Some(true) {
                query.push(" DESC");
            } else {
                query.push(" ASC");
            }
        }

        if let Some(limit) = self.limit {
            query.push(" LIMIT ").push_bind(limit);
        }
        if let Some(offset) = self.offset {
            query.push(" OFFSET ").push_bind(offset);
        }
    }
}

fn converter_string_to_datetime(
    year: Option<&String>,
    month: Option<&String>,
) -> (Option<NaiveDateTime>, Option<NaiveDateTime>) {
    if year.is_none() {
        return (None, None);
    }

    let y = year.unwrap().parse::<i32>().unwrap();
    let start = NaiveDateTime::new(
        chrono::NaiveDate::from_ymd_opt(y, 1, 1).unwrap(),
        chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
    );

    if let Some(m) = month {
        let month_num = m.parse::<u32>().unwrap();
        return (
            start.checked_add_months(Months::new(month_num - 1)),
            start.checked_add_months(Months::new(month_num)),
        );
    }

    (Some(start), start.checked_add_months(Months::new(12)))
}
