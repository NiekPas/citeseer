use std::collections::HashMap;

use crate::reference::Reference;

pub fn parse_bibtex(bibtex: String) -> Result<Vec<Reference>, String> {
    let mut references: Vec<Reference> = Vec::new();
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut key: String = String::new();

    for line in bibtex.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with('@') {
            key = line.split('{').collect::<Vec<&str>>()[1]
                .split(',')
                .collect::<Vec<&str>>()[0]
                .to_string();
        } else if line.starts_with('}') {
            let reference = Reference::new(key.clone(), fields.clone());
            references.push(reference);
            fields.clear();
        } else {
            let field = line.split('=').collect::<Vec<&str>>();
            let key = field[0].trim().to_string();
            let value = field[1].trim().to_string();
            fields.insert(key, value);
        }
    }

    Ok(references)
}
