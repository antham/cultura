use std::error::Error;

use scraper::Html;
use scraper::Selector;
use serde::Deserialize;
use serde::Serialize;

use super::Crawler;

#[derive(Serialize, Deserialize, Clone)]
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

#[typetag::serde]
impl Crawler for Wikipedia {
    fn get_facts(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let body = reqwest::blocking::get(&self.url)?;

        let fragment = Html::parse_document(body.text()?.as_str());
        let selector = Selector::parse(r#"div[id="mw-content-text"] ul li"#).unwrap();
        Ok(fragment
            .select(&selector)
            .map(|e| {
                e.text()
                    .into_iter()
                    .fold(String::new(), |acc: String, e: &str| acc.to_owned() + e)
            })
            .filter(|e| e.starts_with("..."))
            .map(|e| e.replace("...", "Did you know"))
            .collect::<Vec<String>>())
    }

    fn get_id(&self) -> String {
        "dyk".to_string()
    }
}
