use scraper::Html;
use scraper::Selector;

const DYK_URL: &str = "https://en.wikipedia.org/wiki/Wikipedia:Recent_additions";

pub fn get_dyk_facts() -> Result<Vec<String>, String> {
    match reqwest::blocking::get(DYK_URL) {
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
