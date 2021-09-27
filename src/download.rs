use crate::request::Downloadable;

pub async fn fetch(request: &dyn Downloadable) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(request.url().as_str()).await?.text().await?;
    Ok(body)
}
