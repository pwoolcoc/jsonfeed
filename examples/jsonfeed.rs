extern crate jsonfeed;
extern crate reqwest;

const URL: &'static str = "https://jsonfeed.org/feed.json";

fn main() {
    let r = reqwest::get(URL).unwrap();
    let feed = jsonfeed::from_reader(r).unwrap();

    let url = match feed.feed_url {
        Some(ref url) => url.to_string(),
        None => "<no url>".to_string(),
    };

    println!("Items for the '{} ({})' feed:", feed.title, url);

    for item in &feed.items {
        let title = match item.title {
            Some(ref title) => title.to_string(),
            None => "<no title>".to_string(),
        };
        println!("\t{:?}", title);
    }
}

