use chrono::{DateTime, Duration, Utc};

pub fn get_current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn add_seconds(date: DateTime<Utc>, seconds: i64) -> DateTime<Utc> {
    date + Duration::seconds(seconds)
}

pub fn add_days(date: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    date + Duration::days(days)
}

pub fn add_months(date: DateTime<Utc>, months: i64) -> DateTime<Utc> {
    let naive_date = date.naive_utc();
    let year = naive_date.year() as i32;
    let month = naive_date.month() as i32 + months as i32;
    
    let new_year = year + (month - 1) / 12;
    let new_month = ((month - 1) % 12) + 1;
    
    let new_date = chrono::NaiveDate::from_ymd_opt(
        new_year as i32,
        new_month as u32,
        naive_date.day(),
    )
    .unwrap_or_else(|| {
        // Handle edge case (e.g., January 31 + 1 month = February 28/29)
        let last_day = chrono::NaiveDate::from_ymd_opt(
            new_year as i32,
            new_month as u32,
            1,
        )
        .unwrap()
        .with_day(1)
        .unwrap()
        .checked_add_months(chrono::Months::new(1))
        .unwrap()
        .pred()
        .day();
        
        chrono::NaiveDate::from_ymd_opt(
            new_year as i32,
            new_month as u32,
            last_day,
        )
        .unwrap()
    });
    
    DateTime::<Utc>::from_naive_utc_and_offset(
        new_date.and_time(naive_date.time()),
        Utc,
    )
}

pub fn format_date(date: DateTime<Utc>, format: &str) -> String {
    date.format(format).to_string()
}

pub fn parse_date(date_str: &str, format: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_str(date_str, format)
        .map_err(|e| format!("Error parsing date: {}", e))
        .map(|dt| dt.with_timezone(&Utc))
}
