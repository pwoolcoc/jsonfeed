//! At the time of writing, these feeds are listed on the jsonfeed.org
//! website as "sites that have json feed feeds," so let's try to parse
//! them all

extern crate jsonfeed;
extern crate reqwest;

const FEEDS: &'static [(&'static str, &'static str)] = &[
    ("https://jsonfeed.org/feed.json", "JSON Feed Project Page"),
    ("http://shapeof.com/feed.json", "Shape Of"),
    ("http://flyingmeat.com/blog/feed.json", "Flying Meat"),
    ("http://maybepizza.com/feed.json", "Maybe Pizza?"), 
    ("https://daringfireball.net/feeds/json", "Daring Fireball"),
    ("http://hypercritical.co/feeds/main.json","Hypercritical"),
    ("http://inessential.com/feed.json","inessential"),
    ("https://manton.org/feed/json", "Manton Reece"),
    ("https://micro.blog/feeds/manton.json", "Micro.blog timeline"),
    ("http://timetable.manton.org/feed.json", "Timetable (podcast)"),
    ("http://therecord.co/feed.json", "The Record (podcast)"),
    ("http://www.allenpike.com/feed.json", "Allen Pike"),
];

fn main() {
    for &(url, name) in FEEDS {
        let r = reqwest::get(url).unwrap();
        let feed = match jsonfeed::from_reader(r) {
            Ok(f) => f,
            Err(_) => {
                println!("Error retrieving '{}' feed", name);
                continue;
            },
        };

        let url = match feed.feed_url {
            Some(ref url) => url.to_string(),
            None => "<no url>".to_string(),
        };

        println!("First 10 items from the '{} ({})' feed:", feed.title, url);

        for item in feed.items.iter().take(10) {
            let title = match item.title {
                Some(ref title) => title.to_string(),
                None => "<no title>".to_string(),
            };

            let url = match item.url {
                Some(ref url) => url.to_string(),
                None => "<no url>".to_string(),
            };
            println!("\t[{}]({})", title, url);
        }
        print!("\n");
    }
}

