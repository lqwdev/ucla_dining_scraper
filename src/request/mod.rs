pub mod item_request;
pub mod menu_request;

use async_trait::async_trait;

#[async_trait]
pub trait Downloadable {

    fn url(&self) -> String;

    async fn download(&self) -> Result<String, Box<dyn std::error::Error>> {
        let body = reqwest::get(self.url().as_str()).await?.text().await?;
        Ok(body)
    }
}
