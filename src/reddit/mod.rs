use scraper::Html;
use scraper::Selector;

const TIL_URL: &str = "https://www.reddit.com/r/todayilearned/new";

pub fn get_til_facts() -> Result<Vec<String>, String> {
    match reqwest::blocking::get(TIL_URL) {
        Ok(body) => {
            let fragment = Html::parse_document(body.text().unwrap().as_str());
            let selector = Selector::parse(r#"a[data-click-id="body"]"#).unwrap();

            Ok(fragment
                .select(&selector)
                .map(|e| {
                    e.text()
                        .into_iter()
                        .fold(String::new(), |acc: String, e: &str| acc.to_owned() + e)
                })
                .collect::<Vec<String>>())
        }
        Err(_) => Err(String::from("cannot fetch the data for reddit")),
    }
}
