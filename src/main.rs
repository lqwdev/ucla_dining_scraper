mod date;
mod model;
mod parse;
mod request;

use clap::{App, Arg, ArgMatches};
use model::storage::Storage;
use model::{DateMenu, RestaurantMenu};
use parse::parse_item;
use request::Downloadable;
use std::fs::OpenOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("UCLA Menu Scraper")
        .version("1.0.0")
        .author("Qingwei Lan <qingweilandeveloper@gmail.com>")
        .about("Scrapes UClA dining website for menus and downloads the data")
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Download all menus starting from current date")
                .conflicts_with("date"),
        )
        .arg(
            Arg::with_name("with-details")
                .short("d")
                .long("with-details")
                .help("Download menus along all item details"),
        )
        .arg(
            Arg::with_name("date")
                .long("date")
                .required_unless("all")
                .takes_value(true)
                .help("Specify the date (YYYY-MM-DD) for which menu to download"),
        )
        .arg(
            Arg::with_name("save")
                .long("save")
                .takes_value(true)
                .conflicts_with("save-pretty")
                .help("Save the downloaded data on disk in min JSON format"),
        )
        .arg(
            Arg::with_name("save-pretty")
                .long("save-pretty")
                .takes_value(true)
                .conflicts_with("save")
                .help("Save the downloaded data on disk in long pretty JSON format"),
        )
        .get_matches();

    run(&app).await
}

async fn run(app: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let dates = get_dates(app);
    for date in dates {
        print!("Fetching menus for {} ... \t", date);
        if let Ok(menu) = request::download_menus(date).await {
            println!("[done]");
            if let Ok(()) = save(app, &menu) {
                println!("[done]");
            } else {
                println!("[FAILED]");
            }
        } else {
            println!("[FAILED]");
        }
    }
    Ok(())
}

fn save(app: &ArgMatches, menu: &DateMenu) -> Result<(), Box<dyn std::error::Error>> {
    if app.is_present("save") || app.is_present("save-pretty") {
        // Get directory for which to save downloaded data
        let dir = {
            if app.is_present("save") {
                app.value_of("save").unwrap()
            } else {
                app.value_of("save-pretty").unwrap()
            }
        };
        print!("Storing menus for {} on disk to {} ... \t", menu.date, dir);

        save_json(&menu, dir, app.is_present("save-pretty"))?;
    }

    Ok(())
}

fn save_json(menu: &DateMenu, dir: &str, pretty: bool) -> Result<(), Box<dyn std::error::Error>> {
    let suffix = if pretty { "-pretty" } else { "" };
    let filename = format!("{}{}", menu.date, suffix);
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(format!("{}/{}", dir, filename))?;

        if pretty {
        serde_json::to_writer_pretty(file, &menu.to_json())?;
    } else {
        serde_json::to_writer(file, &menu.to_json_min())?;
    }

    Ok(())
}

async fn inflate_item_details(menu: &mut RestaurantMenu) -> Result<(), Box<dyn std::error::Error>> {
    // Download all item details and inflate placeholders in Menu object
    for section in &mut menu.sections {
        for item in &mut section.items {
            item.set_details(parse_item::parse(
                item.details_request().download().await?.as_str(),
            ))
        }
    }
    Ok(())
}

fn get_dates(app: &ArgMatches) -> Vec<String> {
    // Get all menu requests starting from today until a week later
    if app.is_present("all") {
        return date::get_all_dates();
    } else {
        let date = app.value_of("date").unwrap();
        return vec![date.to_string()];
    }
}
