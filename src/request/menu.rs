use crate::model::{MealEnum, RestaurantEnum};
use crate::request::Downloadable;
use async_trait::async_trait;
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub struct MenuRequest {
    pub date: String,
    pub restaurant: RestaurantEnum,
    pub meal: MealEnum,
}

impl MenuRequest {
    fn new(date: String, restaurant: RestaurantEnum, meal: MealEnum) -> Self {
        MenuRequest {
            date: date,
            restaurant: restaurant,
            meal: meal,
        }
    }
}

#[async_trait]
impl Downloadable for MenuRequest {
    fn url(&self) -> String {
        format!(
            "http://menu.dining.ucla.edu/Menus/{}/{}/{}",
            self.restaurant.url_name(),
            self.date,
            self.meal.url_name()
        )
    }
}

/// Get all menu requests for a list of specific dates
pub fn menu_requests_for_dates(dates: Vec<String>) -> Vec<MenuRequest> {
    for date in &dates {
        verify_date(date);
    }

    RestaurantEnum::iter()
        .cartesian_product(dates)
        .cartesian_product(MealEnum::iter())
        .map(|((res, date), meal)| MenuRequest::new(date, res, meal))
        .collect()
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
            MenuRequest::new(
                "2021-09-28".into(),
                RestaurantEnum::BruinPlate,
                MealEnum::Breakfast
            )
            .url(),
            "http://menu.dining.ucla.edu/Menus/BruinPlate/2021-09-28/Breakfast",
        );
        assert_eq!(
            MenuRequest::new(
                "2021-09-28".into(),
                RestaurantEnum::DeNeve,
                MealEnum::Breakfast
            )
            .url(),
            "http://menu.dining.ucla.edu/Menus/DeNeve/2021-09-28/Breakfast",
        );
        assert_eq!(
            MenuRequest::new(
                "2021-09-26".into(),
                RestaurantEnum::Epicuria,
                MealEnum::Lunch
            )
            .url(),
            "http://menu.dining.ucla.edu/Menus/Epicuria/2021-09-26/Lunch",
        );
    }

    #[test]
    fn test_menu_requests_for_dates() {
        assert_eq!(
            menu_requests_for_dates(vec!["2020-08-18".into(), "2020-08-19".into()]),
            vec![
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::BruinPlate,
                    MealEnum::Breakfast
                ),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::BruinPlate,
                    MealEnum::Lunch
                ),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::BruinPlate,
                    MealEnum::Dinner
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::BruinPlate,
                    MealEnum::Breakfast
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::BruinPlate,
                    MealEnum::Lunch
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::BruinPlate,
                    MealEnum::Dinner
                ),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::DeNeve,
                    MealEnum::Breakfast
                ),
                MenuRequest::new("2020-08-18".into(), RestaurantEnum::DeNeve, MealEnum::Lunch),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::DeNeve,
                    MealEnum::Dinner
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::DeNeve,
                    MealEnum::Breakfast
                ),
                MenuRequest::new("2020-08-19".into(), RestaurantEnum::DeNeve, MealEnum::Lunch),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::DeNeve,
                    MealEnum::Dinner
                ),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::Epicuria,
                    MealEnum::Breakfast
                ),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::Epicuria,
                    MealEnum::Lunch
                ),
                MenuRequest::new(
                    "2020-08-18".into(),
                    RestaurantEnum::Epicuria,
                    MealEnum::Dinner
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::Epicuria,
                    MealEnum::Breakfast
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::Epicuria,
                    MealEnum::Lunch
                ),
                MenuRequest::new(
                    "2020-08-19".into(),
                    RestaurantEnum::Epicuria,
                    MealEnum::Dinner
                ),
            ]
        );
    }
}
