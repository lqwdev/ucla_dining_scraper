use crate::request::Downloadable;
use async_trait::async_trait;

#[derive(Debug, PartialEq)]
pub struct ItemRequest {
    pub id: String,
}

impl ItemRequest {
    fn new(id: String) -> Self {
        ItemRequest {
            id: id
        }
    }
}

#[async_trait]
impl Downloadable for ItemRequest {
    fn url(&self) -> String {
        format!("http://menu.dining.ucla.edu/Recipes/{}/1", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_request_url() {
        assert_eq!(
            ItemRequest::new("977026".to_string()).url(),
            "http://menu.dining.ucla.edu/Recipes/977026/1"
        );
        assert_eq!(
            ItemRequest::new("977085".to_string()).url(),
            "http://menu.dining.ucla.edu/Recipes/977085/1"
        );
        assert_eq!(
            ItemRequest::new("141301".to_string()).url(),
            "http://menu.dining.ucla.edu/Recipes/141301/1"
        );
    }
}
