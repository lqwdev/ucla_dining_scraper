use crate::model::item::{Item, ItemDetails};
use crate::model::menu::{Menu, Section};
use std::fmt;

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

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Section: {}\n", self.name)?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ID: {}", self.id)?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Recipe Link: {}", self.recipe_link)?;
        writeln!(
            f,
            "  {}",
            match &self.details {
                Some(details) => details.to_string(),
                None => "Details Not Downloaded".to_string(),
            }
        )
    }
}

impl fmt::Display for ItemDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Description: {}",
            self.description
                .as_ref()
                .unwrap_or(&"no description".to_string())
        )?;
        writeln!(
            f,
            "  Ingredients: {}",
            self.ingredients
                .as_ref()
                .unwrap_or(&"no ingredients".to_string())
        )?;
        writeln!(
            f,
            "  Allergens: {}",
            self.allergens
                .as_ref()
                .unwrap_or(&"no allergens".to_string())
        )
    }
}
