use crate::request::Downloadable;

#[derive(Debug, PartialEq)]
pub struct ItemRequest {
    pub id: String,
    pub url: String,
}

impl Downloadable for ItemRequest {
    fn url(&self) -> String {
        self.url.clone()
    }
}
