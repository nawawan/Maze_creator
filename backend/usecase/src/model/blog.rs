use serde::{Deserialize, Serialize};
use chrono::{DateTime, Months, NaiveDate, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BlogStatus {
    Draft,
    Published,
}
    
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blog {
    pub id: u32,
    pub title: String,
    pub content_key: BlogStatus,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct BlogFilter {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

impl BlogFilter {
    pub fn new(year: Option<&String>, month: Option<&String>) -> Self {
        let (start, end) = converter_string_to_datetime(year, month);
        Self { start, end }
    }
}

fn converter_string_to_datetime(year: Option<&String>, month: Option<&String>) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
    if year.is_none() {
       return (None, None);
    }

    let y = year.unwrap().parse::<i32>().unwrap();
    let start = DateTime::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(y, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(), Utc);

    if let Some(m) = month {
        let month_num = m.parse::<u32>().unwrap();
        return (start.checked_add_months(Months::new(month_num - 1)), start.checked_add_months(Months::new(month_num)));
    }

    (Some(start), start.checked_add_months(Months::new(12)))
}