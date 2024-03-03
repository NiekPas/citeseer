use std::collections::HashMap;

use crate::reference::{Reference, ReferenceType};

pub fn parse_bibtex(bibtex: String) -> Result<Vec<Reference>, String> {
    let mut references: Vec<Reference> = Vec::new();
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut key: String = String::new();
    let mut maybe_reference_type: Result<ReferenceType, String> =
        Err("Failed to parse reference type".to_owned());

    for line in bibtex.lines() {
        // Skip blank lines
        if line.trim().is_empty() {
            continue;
        }
        // @article{veerman2012,
        // ["@article", "veerman2012,"]
        // If the line starts with '@', we're parsing a key
        if line.starts_with('@') {
            let split = line.split('{').collect::<Vec<&str>>();
            maybe_reference_type = parse_reference_type(split[0]);
            key = split[1].split(',').collect::<Vec<&str>>()[0].to_string();
        // If the line starts with '}', we're done parsing this reference and can move on
        } else if line.starts_with('}') {
            // Parsing the reference type ("@article", etc.) may have failed, in which case we return an Err
            match maybe_reference_type {
                Ok(ref reference_type) => {
                    let reference =
                        Reference::new(key.clone(), reference_type.to_owned(), fields.clone());
                    references.push(reference);
                    fields.clear();
                }
                Err(err) => return Err(err.to_owned()),
            }
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

fn parse_reference_type(str: &str) -> Result<ReferenceType, String> {
    ReferenceType::try_from(strip_optional_prefix(str, "@"))
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
