use std::collections::HashMap;

#[derive(Debug)]
pub struct Reference {
    pub key: String,
    pub fields: HashMap<String, String>,
}

struct Author {
    first_name: String,
    last_name: String,
}

impl Author {
    fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
        }
    }
}

impl Reference {
    pub fn new(key: String, fields: HashMap<String, String>) -> Reference {
        Reference { key, fields }
    }

    pub fn as_array(&self) -> [Option<String>; 4] {
        // TODO unwrap this
        let title: Option<String> = self.fields.get("title").cloned();
        let author: Option<String> = self.formatted_author().to_owned();
        let year: Option<String> = self.fields.get("year").cloned();

        [Some(self.key.to_owned()), author, year, title]
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn title(&self) -> Option<&String> {
        self.fields.get("title")
    }

    pub fn author(&self) -> Option<&String> {
        self.fields.get("author")
    }

    pub fn formatted_author(&self) -> Option<String> {
        self.fields
            .get("author")
            .and_then(extract_authors)
            .map(|authors| authors.iter().map(format_author).collect::<Vec<String>>())
            .map(|authors| authors.join("; "))
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.fields == other.fields
    }
}

impl Eq for Reference {}

impl PartialOrd for Reference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a1 = self.fields.get("author");
        let a2 = other.fields.get("author");
        a1.partial_cmp(&a2)
    }
}

impl Ord for Reference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a1 = self.fields.get("author");
        let a2 = other.fields.get("author");
        a1.cmp(&a2)
    }
}

pub fn _example_references() -> [Reference; 2] {
    let mut fields1: HashMap<String, String> = HashMap::new();
    let mut fields2: HashMap<String, String> = HashMap::new();

    fields1.insert(String::from("author"), String::from("Smith, John"));
    fields1.insert(String::from("title"), String::from("Programming Language"));
    fields1.insert(String::from("year"), String::from("2021"));
    fields1.insert(String::from("journal"), String::from("Rust Journal"));
    fields1.insert(String::from("volume"), String::from("5"));
    fields1.insert(String::from("number"), String::from("2"));
    fields1.insert(String::from("pages"), String::from("100-120"));
    fields1.insert(String::from("month"), String::from("July"));
    fields1.insert(String::from("note"), String::from("a sample reference."));

    fields2.insert(String::from("author"), String::from("Doe, Jane"));
    fields2.insert(String::from("title"), String::from("Introduction to Rust"));
    fields2.insert(String::from("year"), String::from("2022"));
    fields2.insert(String::from("journal"), String::from("Rust Gazette"));
    fields2.insert(String::from("volume"), String::from("7"));
    fields2.insert(String::from("number"), String::from("1"));
    fields2.insert(String::from("pages"), String::from("50-70"));
    fields2.insert(String::from("month"), String::from("January"));
    fields2.insert(
        String::from("note"),
        String::from("Another sample reference."),
    );

    let reference1: Reference = Reference {
        key: String::from("smith2021"),
        fields: fields1,
    };

    let reference2: Reference = Reference {
        key: String::from("doe2022"),
        fields: fields2,
    };

    return [reference1, reference2];
}

fn extract_authors(author: &String) -> Option<Vec<Author>> {
    author
        .split(" and ")
        .map(|author| author.split(","))
        .map(|lastname_firstname| {
            let vec = lastname_firstname.collect::<Vec<&str>>();
            if let Some((last_name, first_name)) =
                vec.get(0).and_then(|last_name| match vec.get(1) {
                    Some(first_name) => Some((last_name, first_name)),
                    None => None,
                })
            {
                Some(Author::new(
                    first_name.trim().to_owned(),
                    last_name.trim().to_owned(),
                ))
                // Some((first_name.trim(), last_name.trim()))
            } else {
                None
            }
        })
        .collect()
}

pub fn _search_references<'a>(
    references: &'a Vec<Reference>,
    search_string: &'a String,
) -> Vec<&'a Reference> {
    references
        .iter()
        .filter(|reference| _contains_string(reference, search_string))
        .collect()
}

fn _contains_string(reference: &Reference, string: &String) -> bool {
    reference.key.contains(string)
        || reference
            .fields
            .values()
            .any(|value| value.contains(string))
}

fn format_author(author: &Author) -> String {
    format!("{}, {}", author.last_name, author.first_name)
}
