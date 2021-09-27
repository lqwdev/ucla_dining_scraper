use crate::request::Downloadable;
use async_trait::async_trait;

#[derive(Debug, PartialEq)]
pub struct ItemRequest {
    pub id: String,
    pub url: String,
}

#[async_trait]
impl Downloadable for ItemRequest {
    fn url(&self) -> String {
        self.url.clone()
    }
}
