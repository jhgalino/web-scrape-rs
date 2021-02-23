## web-scrape-rs

A basic web scraper written in Rust. Outputs an `output.html` that contains the html code.

### Building

Make sure that you have Rust installed on your computer. Instructions for installing Rust can be found [here](https://www.rust-lang.org/tools/install). If you have Rust installed, then:

1. Clone this repository
2. Open a terminal inside the clone repository
3. Enter `cargo build --release`

### Usage

Just run the executable with the flag `-u` or `--url` followed by the url of the site you want to scrape

**Example**

`web-scrape-rs -u https:\\google.com`
