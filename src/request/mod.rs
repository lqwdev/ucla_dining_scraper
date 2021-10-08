pub mod item_request;
pub mod menu_request;

use crate::model::storage::Storage;
use crate::model::DateMenu;
use crate::parse::parse_menu;
use async_trait::async_trait;

#[async_trait]
pub trait Downloadable {
    fn url(&self) -> String;

    async fn download(&self) -> Result<String, Box<dyn std::error::Error>> {
        let body = reqwest::get(self.url().as_str()).await?.text().await?;
        Ok(body)
    }
}

pub async fn download_menus(date: String) -> Result<DateMenu, Box<dyn std::error::Error>> {
    let requests = menu_request::menu_requests_for_dates(vec![date.clone()]);
    let mut date_menu = DateMenu {
        date: date.clone(),
        restaurants: Vec::new(),
    };

    for request in requests {
        if let Ok(body) = request.download().await {
            let menu = parse_menu::parse(body.as_str(), &request);
            date_menu.add_restaurant(menu);
        }
    }

    Ok(date_menu)
}
