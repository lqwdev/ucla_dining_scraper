use crate::model::item::Item;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Menu {
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
