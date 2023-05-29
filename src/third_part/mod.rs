use std::{collections::HashMap, error::Error};

use dyn_clone::DynClone;

#[typetag::serde(tag = "provider")]
pub trait Crawler: DynClone {
    fn get_facts(&self) -> Result<Vec<String>, Box<dyn Error>>;
    fn get_id(&self) -> String;
}

dyn_clone::clone_trait_object!(Crawler);

pub mod reddit;
pub mod wikipedia;

pub fn get_available_providers() -> HashMap<String, Box<dyn Crawler>> {
    let mut map_providers: HashMap<String, Box<dyn Crawler>> = HashMap::new();
    let providers: Vec<Box<dyn Crawler>> = vec![
        Box::new(reddit::TIL::new()),
        Box::new(wikipedia::DYK::new()),
    ];
    providers.into_iter().for_each(|p| {
        map_providers.insert(p.get_id(), p);
    });
    map_providers
}
