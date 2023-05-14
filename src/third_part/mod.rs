use std::collections::HashMap;

pub trait Crawler {
    fn get_facts(&self) -> Result<Vec<String>, String>;
    fn get_id(&self) -> String;
}

pub mod reddit;
pub mod wikipedia;

pub fn get_available_providers() -> HashMap<String, Box<dyn Crawler>> {
    let mut map_providers: HashMap<String, Box<dyn Crawler>> = HashMap::new();
    let providers: Vec<Box<dyn Crawler>> = vec![
        Box::new(reddit::Reddit::new()),
        Box::new(wikipedia::Wikipedia::new()),
    ];
    providers.into_iter().for_each(|p| {
        map_providers.insert(p.get_id(), p);
    });
    map_providers
}
