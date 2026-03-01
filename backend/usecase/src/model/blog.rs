use serde::{Deserialize, Serialize};
use chrono::{Months, NaiveDateTime};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BlogStatus {
    Draft,
    Published,
}
    
#[derive(Debug, Clone)]
pub struct Blog {
    pub id: Uuid,
    pub title: String,
    pub content_key: String,
    pub status: BlogStatus,
}

#[derive(Debug, Clone)]
pub struct BlogRequest {
    pub title: String,
    pub content: String,
}

pub struct BlogFilter {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

impl BlogFilter {
    pub fn new(year: Option<&String>, month: Option<&String>) -> Self {
        let (start, end) = converter_string_to_datetime(year, month);
        Self { start, end }
    }
}

fn converter_string_to_datetime(year: Option<&String>, month: Option<&String>) -> (Option<NaiveDateTime>, Option<NaiveDateTime>) {
    if year.is_none() {
       return (None, None);
    }

    let y = year.unwrap().parse::<i32>().unwrap();
    let start = NaiveDateTime::new(chrono::NaiveDate::from_ymd_opt(y, 1, 1).unwrap(), chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap());

    if let Some(m) = month {
        let month_num = m.parse::<u32>().unwrap();
        return (start.checked_add_months(Months::new(month_num - 1)), start.checked_add_months(Months::new(month_num)));
    }

    (Some(start), start.checked_add_months(Months::new(12)))
}