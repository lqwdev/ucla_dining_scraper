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
        writeln!(f, "  {}", match &self.details {
            Some(details) => details.to_string(),
            None => "Not Downloaded".to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ItemDetails {
    pub description: String,
    pub ingredients: String,
    pub allergens: String,
}

impl fmt::Display for ItemDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Description: {}", self.description)?;
        writeln!(f, "  Ingredients: {}", self.ingredients)?;
        writeln!(f, "  Allergens: {}", self.allergens)
    }
}
