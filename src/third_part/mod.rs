pub trait Crawler {
    fn get_facts(&self) -> Result<Vec<String>, String>;
    fn get_id(&self) -> String;
}

pub mod reddit;
pub mod wikipedia;
