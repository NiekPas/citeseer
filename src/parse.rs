use std::collections::HashMap;

use crate::reference::Reference;

pub fn parse_bibtex(bibtex: String) -> Result<Vec<Reference>, String> {
    let mut references: Vec<Reference> = Vec::new();
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut key: String = String::new();

    for line in bibtex.lines() {
        // Skip blank lines
        if line.trim().is_empty() {
            continue;
        }
        // If the line starts with '@', we're parsing a key
        if line.starts_with('@') {
            key = line.split('{').collect::<Vec<&str>>()[1]
                .split(',')
                .collect::<Vec<&str>>()[0]
                .to_string();
        // If the line starts with '}', we're done parsing this reference and can move on
        } else if line.starts_with('}') {
            let reference = Reference::new(key.clone(), fields.clone());
            references.push(reference);
            fields.clear();
        // Otherwise, we're parsing a field, which is the form "author = {hello}"
        } else {
            let field = line.split('=').collect::<Vec<&str>>();
            // If splitting on '=' failed, skip this field
            if field.len() < 2 {
                continue;
            }
            let key = field[0].trim().to_string();
            let value = strip_optional_suffix(
                strip_optional_suffix(strip_optional_prefix(field[1].trim(), "{"), ","),
                "}",
            )
            .to_string();
            fields.insert(key, value);
        }
    }

    Ok(references)
}

fn strip_optional_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    if let Some(s1) = s.strip_prefix(prefix) {
        s1
    } else {
        s
    }
}

fn strip_optional_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    if let Some(s1) = s.strip_suffix(suffix) {
        s1
    } else {
        s
    }
}
