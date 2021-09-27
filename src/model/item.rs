use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub recipe_link: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ID: {}", self.id)?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Recipe Link: {}", self.recipe_link)
    }
}
