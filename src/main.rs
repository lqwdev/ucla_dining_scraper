mod date;
mod model;
mod parse;
mod request;

use model::Page;
use request::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let requests = request::menu_requests();
    for request in requests {
        fetch_and_print(&request).await?
    }

    Ok(())
}

async fn fetch_and_print(request: &Request) -> Result<(), Box<dyn std::error::Error>> {
    let url = request.url();
    println!("Fetching: {}", url);

    let page = fetch_and_parse(request).await?;
    println!("{}", page);

    Ok(())
}

async fn fetch_and_parse(request: &Request) -> Result<Page, Box<dyn std::error::Error>> {
    let body = reqwest::get(request.url().as_str()).await?.text().await?;
    let page = parse::parse(body.as_str(), &request);
    Ok(page)
}
