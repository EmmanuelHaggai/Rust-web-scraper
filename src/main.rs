// A simple web scraper in Rust
fn main() {
    // Make a blocking HTTP GET request to IMDb's top 50 movies page and retrieve the response
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=50",
    )
    .unwrap()
    .text()
    .unwrap();

    // Parse the HTML content into a scraper Html document
    let document = scraper::Html::parse_document(&response);

    // Define a CSS selector for extracting movie titles
    let title_selector = scraper::Selector::parse("h3.lister-item-header > a").unwrap();

    // Extract movie titles by selecting elements matching the CSS selector and mapping their inner HTML
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    // Combine the titles with corresponding numbers and print them
    titles
        .zip(1..51)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}

