use crate::request::item_request::ItemRequest;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub recipe_link: String,
    pub details: Option<ItemDetails>,
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
