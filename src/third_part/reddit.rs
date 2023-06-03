use std::error::Error;

use scraper::Html;
use scraper::Selector;
use serde::Deserialize;
use serde::Serialize;

use super::Crawler;

#[derive(Serialize, Deserialize, Clone)]
pub struct TIL {
    #[serde(skip)]
    url: String,
}

impl TIL {
    pub fn new() -> TIL {
        TIL {
            url: "https://www.reddit.com/r/todayilearned/new".to_string(),
        }
    }
}

#[typetag::serde]
impl Crawler for TIL {
    fn get_facts(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
            )
            .build()?;
        let body = client.get(&self.url).send()?;
        let fragment = Html::parse_document(body.text()?.as_str());
        let selector = Selector::parse(r#"a[data-click-id="body"]"#).unwrap();

        Ok(fragment
            .select(&selector)
            .map(|e| {
                e.text()
                    .into_iter()
                    .fold(String::new(), |acc: String, e: &str| acc.to_owned() + e)
                    .replace("TIL", "Today I learned")
            })
            .collect::<Vec<String>>())
    }

    fn get_id(&self) -> String {
        "TIL".to_string()
    }
}
