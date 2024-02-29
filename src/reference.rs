use std::{collections::HashMap, str::Split};

#[derive(Debug, Clone)]
pub struct Reference {
    pub key: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
enum AuthorName {
    FirstNameLastName {
        first_name: String,
        last_name: String,
    },
    FullName(String),
}

#[derive(Debug, PartialEq)]
struct Author {
    name: AuthorName,
}

impl Author {
    fn new(first_name: String, last_name: String) -> Self {
        Self {
            name: AuthorName::FirstNameLastName {
                first_name,
                last_name,
            },
        }
    }

    fn new_from_full_name(full_name: String) -> Self {
        Self {
            name: AuthorName::FullName(full_name),
        }
    }
}

impl Reference {
    pub fn new(key: String, fields: HashMap<String, String>) -> Reference {
        Reference { key, fields }
    }

    pub fn as_array(&self) -> [Option<String>; 4] {
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

    pub fn year(&self) -> Option<&String> {
        self.fields.get("year")
    }

    pub fn formatted_author(&self) -> Option<String> {
        self.fields
            .get("author")
            .map(extract_authors_from_string)
            .map(|authors| authors.iter().map(format_author).collect::<Vec<String>>())
            .map(|authors| authors.join("; "))
    }

    pub fn to_bibtex(&self) -> String {
        let mut bibtex = format!("@article{{{key},\n", key = self.key);

        for (field, value) in &self.fields {
            bibtex.push_str(&format!("    {} = {{{}}},\n", field, value));
        }

        bibtex.push_str("}\n");

        bibtex
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

fn extract_authors_from_string(authors: &String) -> Vec<Author> {
    return authors
        .split(" and ")
        .map(|author| author.split(","))
        .map(parse_split_author)
        .collect();

    fn parse_split_author(lastname_firstname: Split<&str>) -> Author {
        let vec = lastname_firstname.collect::<Vec<&str>>();
        // If, when we split this author on ',' and `collect()` the results, we have exactly 2 elements,
        // that means we have a last name and a first name. In that case, we simply return those as a new author.
        // ("Doe, John")
        if vec.len() == 2 {
            let last_name = vec[0].trim().to_owned();
            let first_name = vec[1].trim().to_owned();
            Author {
                name: AuthorName::FirstNameLastName {
                    first_name,
                    last_name,
                },
            }
        } else {
            // If not, we proceed by checking if the full name is of the format "John Doe" or "Marieke M.A. Hendriksen" by splitting on whitespace.
            let names: Vec<&str> = vec[0].split_ascii_whitespace().collect();

            // John Doe
            // Marieke M.A. Huuu
            if names.len() > 2 && is_initials(names[1]) {
                let first_name = names[..names.len() - 1].join(" ");
                let last_name = names[names.len() - 1].to_owned();
                return Author::new(first_name, last_name);
            }

            if names.len() == 2 {
                // If so, we assume "Doe" is the last name and "John" is the first name.
                Author::new(names[0].to_string(), names[1].to_string())
            } else {
                // If the full name is not of the format "John Doe", we fall back on returning an AuthorName::FullName.
                Author::new_from_full_name(vec[0].to_string())
            }
        }
    }
}

fn is_initials(str: &str) -> bool {
    if str == "" {
        false
    }
    // If all characters are uppercase letters, we can assume this is an initial
    else if str.chars().all(|c| c.is_alphabetic() && c.is_uppercase()) {
        true
    } else {
        // If not, we check if the string is of the form "M.A.B." by looping over it
        for (i, c) in str.char_indices() {
            // The current character should be a letter for even indices, and a period ('.') for odd indices.
            let is_expected_initial_char = if i % 2 == 0 {
                c.is_alphabetic()
            } else {
                c == '.'
            };

            if !is_expected_initial_char {
                return false;
            }
        }
        true
    }
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
    match author.name {
        AuthorName::FirstNameLastName {
            ref first_name,
            ref last_name,
        } => format!(
            "{}, {}",
            last_name,
            first_name.chars().next().unwrap_or_default()
        ),
        AuthorName::FullName(ref full_name) => full_name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::reference::Reference;
    use std::collections::HashMap;

    fn _example_references() -> [Reference; 2] {
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

    #[test]
    fn test_extract_authors() {
        {
            let test_author = String::from("Smith, John");
            let extracted_authors = extract_authors_from_string(&test_author);
            let expected: Vec<Author> = vec![Author {
                name: AuthorName::FirstNameLastName {
                    first_name: String::from("John"),
                    last_name: String::from("Smith"),
                },
            }];

            assert_eq!(expected, extracted_authors);
        }
        {
            let test_author = String::from("John Smith");
            let extracted_authors = extract_authors_from_string(&test_author);
            let expected: Vec<Author> = vec![Author {
                name: AuthorName::FirstNameLastName {
                    first_name: String::from("John"),
                    last_name: String::from("Smith"),
                },
            }];

            assert_eq!(expected, extracted_authors);
        }
        {
            let test_author = String::from("Juan Pablo Fernández de Calderón García-Iglesias");
            let extracted_authors = extract_authors_from_string(&test_author);
            let expected: Vec<Author> = vec![Author {
                name: AuthorName::FullName(test_author),
            }];

            assert_eq!(expected, extracted_authors);
        }
        {
            let test_author = String::from("Marieke M.A. Hendriksen");
            let extracted_authors = extract_authors_from_string(&test_author);
            let expected: Vec<Author> = vec![Author {
                name: AuthorName::FirstNameLastName {
                    first_name: String::from("Marieke M.A."),
                    last_name: String::from("Hendriksen"),
                },
            }];

            assert_eq!(expected, extracted_authors);
        }
        {
            let test_author = String::from("Hendriksen, Marieke M.A.");
            let extracted_authors = extract_authors_from_string(&test_author);
            let expected: Vec<Author> = vec![Author {
                name: AuthorName::FirstNameLastName {
                    first_name: String::from("Marieke M.A."),
                    last_name: String::from("Hendriksen"),
                },
            }];

            assert_eq!(expected, extracted_authors);
        }
    }

    #[test]
    fn test_is_initials() {
        {
            let test_string = String::from("");
            let result = is_initials(&test_string);
            assert_eq!(false, result);
        }
        {
            let test_string = String::from("A.B.");
            let result = is_initials(&test_string);
            assert_eq!(true, result);
        }
        {
            let test_string = String::from("AB");
            let result = is_initials(&test_string);
            assert_eq!(true, result);
        }
        {
            let test_string = String::from("DABS");
            let result = is_initials(&test_string);
            assert_eq!(true, result);
        }
        {
            let test_string = String::from("d.a.b.s.");
            let result = is_initials(&test_string);
            assert_eq!(true, result);
        }
        {
            let test_string = String::from("Pablo");
            let result = is_initials(&test_string);
            assert_eq!(false, result);
        }
        {
            let test_string = String::from("martin");
            let result = is_initials(&test_string);
            assert_eq!(false, result);
        }
    }
}
