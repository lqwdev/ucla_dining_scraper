mod date;
mod model;
mod parse;
mod request;

use clap::{App, Arg, ArgMatches};
use request::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("UCLA Menu Scraper")
        .version("1.0.0")
        .author("Qingwei Lan <qingweilandeveloper@gmail.com>")
        .about("Scrapes UClA dining website for menus and downloads the data")
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .help("Print the downloaded menus content"),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Download all menus starting from current date")
                .conflicts_with("date"),
        )
        .arg(
            Arg::with_name("date")
                .long("date")
                .required_unless("all")
                .takes_value(true)
                .help("Specify the date (YYYY-MM-DD) for which menu to download"),
        )
        .get_matches();

    run(&app).await
}

async fn run(app: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let requests = parse_requests(app);

    for request in requests {
        print!(
            "{} {} for {} ... \t",
            request.date,
            request.meal.name(),
            request.restaurant.name()
        );

        if let Ok(body) = fetch(&request).await {
            let menu = parse::parse(body.as_str(), &request);
            println!("[done]");

            if should_print(&app) {
                println!("{}", menu);
            }
        }
    }

    Ok(())
}

async fn fetch(request: &Request) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(request.url().as_str()).await?.text().await?;
    Ok(body)
}

fn parse_requests(app: &ArgMatches) -> Vec<Request> {
    if app.is_present("all") {
        return request::menu_requests();
    }

    let date = app.value_of("date").unwrap();
    return request::menu_requests_for_dates(vec![date.into()]);
}

fn should_print(app: &ArgMatches) -> bool {
    app.is_present("print")
}
