mod date;
mod model;
mod parse;
mod request;

use clap::{App, Arg, ArgMatches};
use parse::parse_menu;
use request::menu_request;
use request::menu_request::MenuRequest;
use request::Downloadable;

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
    let requests = get_requests(app);

    for request in requests {
        print!(
            "Fetching {} {} for {} ... \t",
            request.date,
            request.meal.name(),
            request.restaurant.name()
        );

        if let Ok(body) = request.download().await {
            let menu = parse_menu::parse(body.as_str(), &request);
            println!("[done]");

            if app.is_present("print") {
                println!("{}", menu);
            }
        }
    }

    Ok(())
}

fn get_requests(app: &ArgMatches) -> Vec<MenuRequest> {
    // Get all menu requests starting from today until a week later
    if app.is_present("all") {
        return menu_request::get_all_menu_requests();
    }

    // Get menu request for specific date
    let date = app.value_of("date").unwrap();
    return menu_request::menu_requests_for_dates(vec![date.into()]);
}
