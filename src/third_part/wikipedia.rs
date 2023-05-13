use scraper::Html;
use scraper::Selector;

use super::Crawler;

pub struct Wikipedia {
    url: String,
}

impl Wikipedia {
    pub fn new() -> Wikipedia {
        Wikipedia {
            url: "https://en.wikipedia.org/wiki/Wikipedia:Recent_additions".to_string(),
        }
    }
}

impl Crawler for Wikipedia {
    fn get_facts(&self) -> Result<Vec<String>, String> {
        match reqwest::blocking::get(&self.url) {
            Ok(body) => {
                let fragment = Html::parse_document(body.text().unwrap().as_str());
                let selector = Selector::parse(r#"div[id="mw-content-text"] ul li"#).unwrap();
                Ok(fragment
                    .select(&selector)
                    .map(|e| {
                        e.text()
                            .into_iter()
                            .fold(String::new(), |acc: String, e: &str| acc.to_owned() + e)
                    })
                    .filter(|e| e.starts_with("..."))
                    .map(|e| e.replace("...", "Do you know"))
                    .collect::<Vec<String>>())
            }
            Err(_) => Err(String::from("cannot fetch the data for wikipedia")),
        }
    }

    fn get_id(&self) -> String {
        "dyk".to_string()
    }
}