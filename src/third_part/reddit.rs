use scraper::Html;
use scraper::Selector;

use super::Crawler;

pub struct Reddit {
    url: String,
}

impl Reddit {
    pub fn new() -> Reddit {
        Reddit {
            url: "https://www.reddit.com/r/todayilearned/new".to_string(),
        }
    }
}

impl Crawler for Reddit {
    fn get_facts(&self) -> Result<Vec<String>, String> {
        match reqwest::blocking::get(&self.url) {
            Ok(body) => {
                let fragment = Html::parse_document(body.text().unwrap().as_str());
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
            Err(_) => Err(String::from("cannot fetch the data for reddit")),
        }
    }

    fn get_id(&self) -> String {
        "til".to_string()
    }
}
