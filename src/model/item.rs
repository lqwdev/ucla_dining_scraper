use crate::request::item_request::ItemRequest;
use serde::{Deserialize, Serialize};

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
