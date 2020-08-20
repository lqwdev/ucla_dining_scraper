use chrono::{DateTime, Datelike, Duration, FixedOffset, TimeZone, Utc};

pub fn dates() -> Vec<String> {
    let current_date = FixedOffset::west(7 * 3600).from_utc_datetime(&Utc::now().naive_utc());
    dates_from_current_date(current_date)
}

fn dates_from_current_date(current_date: DateTime<FixedOffset>) -> Vec<String> {
    (0..7)
        .map(|num| current_date + Duration::days(num))
        .map(|date| {
            let year = date.year();
            let month = date.month();
            let day = date.day();
            format!("{}-{:02}-{:02}", year, month, day)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_dates() {
        let current_date = FixedOffset::west(7 * 3600)
            .from_utc_datetime(&NaiveDate::from_ymd(2020, 8, 20).and_hms(10, 0, 0));
        assert_eq!(
            dates_from_current_date(current_date),
            vec![
                "2020-08-20",
                "2020-08-21",
                "2020-08-22",
                "2020-08-23",
                "2020-08-24",
                "2020-08-25",
                "2020-08-26",
            ]
        );
    }
}
