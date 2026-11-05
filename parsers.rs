use chrono::NaiveDate;
use crate::Category;

pub fn parse_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%d-%m-%Y")
        .map_err(|_| format!("'{}' is not in the format DD-MM-YYYY", s))
}

pub fn parse_category(s: &str) -> Result<Category, String> {
    s.parse::<Category>().map_err(|_| {
        format!("'{}' is not a valid category. Use 'income' or 'expenses'", s)
    })
}
