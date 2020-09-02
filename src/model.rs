use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Menu {
    pub date: String,
    pub restaurant: Restaurant,
    pub meal: Meal,
    pub sections: Vec<Section>,
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} for {}",
            self.date,
            self.meal.name(),
            self.restaurant.name()
        )?;
        writeln!(f, "---------------------------------")?;

        for section in &self.sections {
            writeln!(f, "{}", section)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub recipe_link: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Recipe Link: {}", self.recipe_link)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Section {
    pub name: String,
    pub items: Vec<Item>,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Section: {}", self.name)?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize, PartialEq, Clone)]
pub enum Restaurant {
    DeNeve,
}

impl Restaurant {
    pub fn name(&self) -> String {
        match self {
            Self::DeNeve => "De Neve".into(),
        }
    }

    pub fn url_name(&self) -> String {
        match self {
            Self::DeNeve => "DeNeve".into(),
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
