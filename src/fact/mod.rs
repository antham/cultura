pub fn generate_random(database_path: String) -> Result<Option<String>, String> {
    let fact = crate::db::Fact::new(&database_path);

    match fact.get_random_fact() {
        Ok(Some((id, data))) => match fact.mark_as_read(id) {
            Ok(_) => Ok(Some(data)),
            Err(e) => Err(e.to_string()),
        },
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_generate_random() {
        let database_name = "generate-random-fact";

        let _ = fs::remove_file(database_name);
        let f = crate::db::Fact::new(database_name);
        f.create(
            "til".to_string(),
            vec!["fact1".to_string(), "fact2".to_string()],
        );

        let mut facts: Vec<String> = vec![];

        let f1 = generate_random(database_name.to_string());
        assert!(f1.is_ok());
        facts.push(f1.ok().unwrap().unwrap());

        let f2 = generate_random(database_name.to_string());
        assert!(f2.is_ok());
        facts.push(f2.ok().unwrap().unwrap());

        let f3 = generate_random(database_name.to_string());
        assert!(f3.is_ok());
        assert!(f3.ok().unwrap() == None);
    }
}
