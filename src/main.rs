use clap::{App, Arg};
use reqwest;
use std::fs;

#[tokio::main]
async fn main() {
    let app = App::new("Web Scrape")
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .takes_value(true),
        )
        .get_matches();
    let res = reqwest::get(app.value_of("url").unwrap())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    fs::write("output.html", res).expect("Error writing to file");
}
