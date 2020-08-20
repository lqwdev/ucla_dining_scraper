use crate::date;
use crate::model::{Meal, Restaurant};
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub date: String,
    pub restaurant: Restaurant,
    pub meal: Meal,
}

impl Request {
    pub fn url(&self) -> String {
        format!(
            "http://menu.dining.ucla.edu/Menus/{}/{}/{}",
            self.restaurant.url_name(),
            self.date,
            self.meal.url_name()
        )
    }
}

pub fn menu_requests() -> Vec<Request> {
    menu_requests_for_dates(date::dates())
}

fn menu_requests_for_dates(dates: Vec<String>) -> Vec<Request> {
    Restaurant::iter()
        .cartesian_product(dates)
        .cartesian_product(Meal::iter())
        .map(|((res, date), meal)| menu_request(date, res, meal))
        .collect()
}

fn menu_request(date: String, restaurant: Restaurant, meal: Meal) -> Request {
    Request {
        date: date,
        restaurant: restaurant,
        meal: meal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_requests_for_dates() {
        assert_eq!(
            menu_requests_for_dates(vec!["2020-08-18".into(), "2020-08-19".into()]),
            vec![
                menu_request("2020-08-18".into(), Restaurant::DeNeve, Meal::Breakfast),
                menu_request("2020-08-18".into(), Restaurant::DeNeve, Meal::Lunch),
                menu_request("2020-08-18".into(), Restaurant::DeNeve, Meal::Dinner),
                menu_request("2020-08-19".into(), Restaurant::DeNeve, Meal::Breakfast),
                menu_request("2020-08-19".into(), Restaurant::DeNeve, Meal::Lunch),
                menu_request("2020-08-19".into(), Restaurant::DeNeve, Meal::Dinner),
            ]
        );
    }
}
