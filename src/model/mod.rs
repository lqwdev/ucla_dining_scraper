pub mod display;
pub mod storage;

use crate::request::item_request::ItemRequest;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DateMenu {
    pub date: String,
    pub restaurants: Vec<Menu>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Menu {
    pub name: Restaurant,
    pub meals: Vec<MenuMeal>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MenuMeal {
    pub name: Meal,
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RestaurantMenu {
    pub date: String,
    pub restaurant: Restaurant,
    pub meal: Meal,
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Section {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, EnumIter, Serialize, Deserialize, PartialEq, Clone)]
pub enum Restaurant {
    BruinPlate,
    DeNeve,
    Epicuria,
}

impl Restaurant {
    pub fn name(&self) -> String {
        match self {
            Self::BruinPlate => "Bruin Plate".into(),
            Self::DeNeve => "De Neve".into(),
            Self::Epicuria => "Epicuria".into(),
        }
    }

    pub fn url_name(&self) -> String {
        match self {
            Self::BruinPlate => "BruinPlate".into(),
            Self::DeNeve => "DeNeve".into(),
            Self::Epicuria => "Epicuria".into(),
        }
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize, PartialEq, Clone)]
pub enum Meal {
    Breakfast,
    Lunch,
    Dinner,
}

impl Meal {
    pub fn name(&self) -> String {
        match self {
            Self::Breakfast => "Breakfast".into(),
            Self::Lunch => "Lunch".into(),
            Self::Dinner => "Dinner".into(),
        }
    }

    pub fn url_name(&self) -> String {
        match self {
            Self::Breakfast => "Breakfast".into(),
            Self::Lunch => "Lunch".into(),
            Self::Dinner => "Dinner".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub recipe_link: String,
    pub details: Option<ItemDetails>,
}

impl Item {
    pub fn details_request(&self) -> ItemRequest {
        ItemRequest {
            id: self.id.clone(),
            url: self.recipe_link.clone(),
        }
    }

    pub fn set_details(&mut self, details: ItemDetails) {
        self.details = Some(details);
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemDetails {
    pub description: Option<String>,
    pub ingredients: Option<String>,
    pub allergens: Option<String>,
}
