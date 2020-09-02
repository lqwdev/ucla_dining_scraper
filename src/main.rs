mod date;
mod model;
mod parse;
mod request;

use request::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let requests = request::menu_requests();
    for request in requests {
        if let Ok(body) = fetch(&request).await {
            let menu = parse::parse(body.as_str(), &request);
            println!("{}", menu)
        }
    }
    Ok(())
}

async fn fetch(request: &Request) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(request.url().as_str()).await?.text().await?;
    Ok(body)
}
