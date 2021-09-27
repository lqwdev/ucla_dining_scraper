use crate::date;
use crate::model::menu::{Meal, Restaurant};
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub struct MenuRequest {
    pub date: String,
    pub restaurant: Restaurant,
    pub meal: Meal,
}

impl MenuRequest {
    pub fn url(&self) -> String {
        format!(
            "http://menu.dining.ucla.edu/Menus/{}/{}/{}",
            self.restaurant.url_name(),
            self.date,
            self.meal.url_name()
        )
    }
}

/// Get all menu requests starting from current date until 7 days later
pub fn get_all_menu_requests() -> Vec<MenuRequest> {
    menu_requests_for_dates(date::get_all_dates())
}

/// Get all menu requests for a list of specific dates
pub fn menu_requests_for_dates(dates: Vec<String>) -> Vec<MenuRequest> {
    for date in &dates {
        verify_date(date);
    }

    Restaurant::iter()
        .cartesian_product(dates)
        .cartesian_product(Meal::iter())
        .map(|((res, date), meal)| menu_request(date, res, meal))
        .collect()
}

fn menu_request(date: String, restaurant: Restaurant, meal: Meal) -> MenuRequest {
    MenuRequest {
        date: date,
        restaurant: restaurant,
        meal: meal,
    }
}

/// Verify that the date string is in correct YYYY-MM-DD format
fn verify_date(date: &String) {
    fn report_invalid_date(date: &String) {
        panic!("Given date must be in YYYY-MM-DD format, got {}", date);
    }

    fn is_numeric(s: &str) -> bool {
        for c in s.to_string().chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        true
    }

    let tokens = date.split("-").collect::<Vec<&str>>();

    // Must have 3 tokens, a token for each date component
    if tokens.len() != 3 {
        report_invalid_date(date);
    }

    // Verify the year component
    if tokens[0].len() != 4 || !is_numeric(tokens[0]) {
        report_invalid_date(date);
    }

    // Verify the month component
    if tokens[1].len() != 2 || !is_numeric(tokens[1]) {
        report_invalid_date(date);
    }

    // Verify the day component
    if tokens[2].len() != 2 || !is_numeric(tokens[2]) {
        report_invalid_date(date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_request_url() {
        assert_eq!(
            menu_request("2021-09-28".into(), Restaurant::BruinPlate, Meal::Breakfast).url(),
            "http://menu.dining.ucla.edu/Menus/BruinPlate/2021-09-28/Breakfast",
        );
        assert_eq!(
            menu_request("2021-09-28".into(), Restaurant::DeNeve, Meal::Breakfast).url(),
            "http://menu.dining.ucla.edu/Menus/DeNeve/2021-09-28/Breakfast",
        );
        assert_eq!(
            menu_request("2021-09-26".into(), Restaurant::Epicuria, Meal::Lunch).url(),
            "http://menu.dining.ucla.edu/Menus/Epicuria/2021-09-26/Lunch",
        );
    }

    #[test]
    fn test_menu_requests_for_dates() {
        assert_eq!(
            menu_requests_for_dates(vec!["2020-08-18".into(), "2020-08-19".into()]),
            vec![
                menu_request("2020-08-18".into(), Restaurant::BruinPlate, Meal::Breakfast),
                menu_request("2020-08-18".into(), Restaurant::BruinPlate, Meal::Lunch),
                menu_request("2020-08-18".into(), Restaurant::BruinPlate, Meal::Dinner),
                menu_request("2020-08-19".into(), Restaurant::BruinPlate, Meal::Breakfast),
                menu_request("2020-08-19".into(), Restaurant::BruinPlate, Meal::Lunch),
                menu_request("2020-08-19".into(), Restaurant::BruinPlate, Meal::Dinner),
                menu_request("2020-08-18".into(), Restaurant::DeNeve, Meal::Breakfast),
                menu_request("2020-08-18".into(), Restaurant::DeNeve, Meal::Lunch),
                menu_request("2020-08-18".into(), Restaurant::DeNeve, Meal::Dinner),
                menu_request("2020-08-19".into(), Restaurant::DeNeve, Meal::Breakfast),
                menu_request("2020-08-19".into(), Restaurant::DeNeve, Meal::Lunch),
                menu_request("2020-08-19".into(), Restaurant::DeNeve, Meal::Dinner),
                menu_request("2020-08-18".into(), Restaurant::Epicuria, Meal::Breakfast),
                menu_request("2020-08-18".into(), Restaurant::Epicuria, Meal::Lunch),
                menu_request("2020-08-18".into(), Restaurant::Epicuria, Meal::Dinner),
                menu_request("2020-08-19".into(), Restaurant::Epicuria, Meal::Breakfast),
                menu_request("2020-08-19".into(), Restaurant::Epicuria, Meal::Lunch),
                menu_request("2020-08-19".into(), Restaurant::Epicuria, Meal::Dinner),
            ]
        );
    }
}
