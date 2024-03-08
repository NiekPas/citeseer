use std::{collections::HashMap, fmt::Display, str::Split};

use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone)]
pub struct Reference {
    pub fields: HashMap<FieldType, String>,
}

#[derive(Debug, Clone)]
pub enum ReferenceType {
    Article,
    Book,
    InBook,
    InCollection,
    InProceedings,
    Manual,
    Mastersthesis,
    Misc,
    Phdthesis,
    Proceedings,
    Techreport,
    Unpublished,
}

impl ToString for ReferenceType {
    fn to_string(&self) -> String {
        match self {
            ReferenceType::Article => String::from("Article"),
            ReferenceType::Book => String::from("Book"),
            ReferenceType::InBook => String::from("InBook"),
            ReferenceType::InCollection => String::from("InCollection"),
            ReferenceType::InProceedings => String::from("InProceedings"),
            ReferenceType::Manual => String::from("Manual"),
            ReferenceType::Mastersthesis => String::from("Mastersthesis"),
            ReferenceType::Misc => String::from("Misc"),
            ReferenceType::Phdthesis => String::from("Phdthesis"),
            ReferenceType::Proceedings => String::from("Proceedings"),
            ReferenceType::Techreport => String::from("Techreport"),
            ReferenceType::Unpublished => String::from("Unpublished"),
        }
    }
}

impl TryFrom<&str> for ReferenceType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "article" => Ok(ReferenceType::Article),
            "book" => Ok(ReferenceType::Book),
            "inbook" => Ok(ReferenceType::InBook),
            "incollection" => Ok(ReferenceType::InCollection),
            "inproceedings" => Ok(ReferenceType::InProceedings),
            "manual" => Ok(ReferenceType::Manual),
            "mastersthesis" => Ok(ReferenceType::Mastersthesis),
            "misc" => Ok(ReferenceType::Misc),
            "phdthesis" => Ok(ReferenceType::Phdthesis),
            "proceedings" => Ok(ReferenceType::Proceedings),
            "techreport" => Ok(ReferenceType::Techreport),
            "report" => Ok(ReferenceType::Techreport),
            "unpublished" => Ok(ReferenceType::Unpublished),
            _ => Err(format!("Failed to parse reference type: {}", value).to_owned()),
        }
    }
}

impl UnicodeWidthStr for ReferenceType {
    fn width<'a>(&'a self) -> usize {
        self.to_string().width()
    }

    fn width_cjk<'a>(&'a self) -> usize {
        self.to_string().width_cjk()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    Abstract,
    Address,
    Annote,
    Author,
    Booktitle,
    Chapter,
    Crossref,
    Day,
    Doi,
    Edition,
    Editor,
    Eprint,
    Howpublished,
    Institution,
    Isbn,
    Issn,
    Journal,
    Key,
    Keywords,
    Month,
    Note,
    Number,
    Organization,
    Pages,
    Pmid,
    Publisher,
    School,
    Series,
    Title,
    Type,
    Url,
    UrlDate,
    Volume,
    Year,
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Abstract => write!(f, "abstract"),
            FieldType::Address => write!(f, "address"),
            FieldType::Annote => write!(f, "annote"),
            FieldType::Author => write!(f, "author"),
            FieldType::Booktitle => write!(f, "booktitle"),
            FieldType::Chapter => write!(f, "chapter"),
            FieldType::Crossref => write!(f, "crossref"),
            FieldType::Day => write!(f, "day"),
            FieldType::Doi => write!(f, "doi"),
            FieldType::Edition => write!(f, "edition"),
            FieldType::Editor => write!(f, "editor"),
            FieldType::Eprint => write!(f, "eprint"),
            FieldType::Howpublished => write!(f, "howpublished"),
            FieldType::Institution => write!(f, "institution"),
            FieldType::Isbn => write!(f, "isbn"),
            FieldType::Issn => write!(f, "issn"),
            FieldType::Journal => write!(f, "journal"),
            FieldType::Key => write!(f, "key"),
            FieldType::Keywords => write!(f, "keywords"),
            FieldType::Month => write!(f, "month"),
            FieldType::Note => write!(f, "note"),
            FieldType::Number => write!(f, "number"),
            FieldType::Organization => write!(f, "organization"),
            FieldType::Pages => write!(f, "pages"),
            FieldType::Pmid => write!(f, "pmid"),
            FieldType::Publisher => write!(f, "publisher"),
            FieldType::School => write!(f, "school"),
            FieldType::Series => write!(f, "series"),
            FieldType::Title => write!(f, "title"),
            FieldType::Type => write!(f, "type"),
            FieldType::Url => write!(f, "url"),
            FieldType::UrlDate => write!(f, "urldate"),
            FieldType::Volume => write!(f, "volume"),
            FieldType::Year => write!(f, "year"),
        }
    }
}

impl TryFrom<&str> for FieldType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "abstract" => Ok(FieldType::Abstract),
            "address" => Ok(FieldType::Address),
            "annote" => Ok(FieldType::Annote),
            "author" => Ok(FieldType::Author),
            "booktitle" => Ok(FieldType::Booktitle),
            "chapter" => Ok(FieldType::Chapter),
            "crossref" => Ok(FieldType::Crossref),
            "day" => Ok(FieldType::Day),
            "doi" => Ok(FieldType::Doi),
            "edition" => Ok(FieldType::Edition),
            "editor" => Ok(FieldType::Editor),
            "eprint" => Ok(FieldType::Eprint),
            "howpublished" => Ok(FieldType::Howpublished),
            "institution" => Ok(FieldType::Institution),
            "isbn" => Ok(FieldType::Isbn),
            "issn" => Ok(FieldType::Issn),
            "journal" => Ok(FieldType::Journal),
            "key" => Ok(FieldType::Key),
            "keywords" => Ok(FieldType::Keywords),
            "month" => Ok(FieldType::Month),
            "note" => Ok(FieldType::Note),
            "number" => Ok(FieldType::Number),
            "issue" => Ok(FieldType::Number),
            "organization" => Ok(FieldType::Organization),
            "pages" => Ok(FieldType::Pages),
            "pmid" => Ok(FieldType::Pmid),
            "publisher" => Ok(FieldType::Publisher),
            "school" => Ok(FieldType::School),
            "series" => Ok(FieldType::Series),
            "title" => Ok(FieldType::Title),
            "type" => Ok(FieldType::Type),
            "url" => Ok(FieldType::Url),
            "urldate" => Ok(FieldType::UrlDate),
            "volume" => Ok(FieldType::Volume),
            "year" => Ok(FieldType::Year),
            _ => Err(format!("Failed to parse field type: {}", value).to_owned()),
        }
    }
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
    pub fn new(
        key: String,
        reference_type: ReferenceType,
        fields: HashMap<FieldType, String>,
    ) -> Reference {
        // This is bad, performance-wise. Let's see if it becomes a problem.
        let mut fields = fields.clone();
        fields.insert(FieldType::Key, key);
        fields.insert(FieldType::Type, reference_type.to_string());
        Reference {
            fields: fields.to_owned(),
        }
    }

    pub fn as_array(&self, visible_headers: &Vec<FieldType>) -> Vec<Option<String>> {
        let retval: Vec<Option<String>> = visible_headers
            .iter()
            .map(|visible_header| self.fields.get(visible_header).cloned())
            .collect();
        retval
    }

    pub fn key(&self) -> Option<&String> {
        self.fields.get(&FieldType::Key)
    }

    pub fn title(&self) -> Option<&String> {
        self.fields.get(&FieldType::Title)
    }

    pub fn year(&self) -> Option<&String> {
        self.fields.get(&FieldType::Year)
    }

    pub fn formatted_author(&self) -> Option<String> {
        self.fields
            .get(&FieldType::Author)
            .map(extract_authors_from_string)
            .map(|authors| authors.iter().map(format_author).collect::<Vec<String>>())
            .map(|authors| authors.join("; "))
    }

    pub fn reference_type(&self) -> Option<&String> {
        self.fields.get(&FieldType::Type)
    }

    pub fn to_bibtex(&self) -> String {
        let opt_key = self.fields.get(&FieldType::Key);
        // If we don't have a 'key', but we do have an author, use that as the key.
        let author = self
            .fields
            .get(&FieldType::Author)
            .map(|s| s.as_ref())
            .unwrap_or("");
        let output_key = match opt_key {
            Some(k) => k.to_owned(),
            None => author.to_owned(),
        };

        let mut bibtex = format!("@article{{{key},\n", key = output_key);

        for (field, value) in &self.fields {
            bibtex.push_str(&format!("    {} = {{{}}},\n", field, value));
        }

        bibtex.push_str("}\n");

        bibtex
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        // TODO I'm not sure if this '==' works now? Probably yes?
        self.fields.get(&FieldType::Key) == self.fields.get(&FieldType::Key)
            && self.fields == other.fields
    }
}

impl Eq for Reference {}

impl PartialOrd for Reference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a1 = self.fields.get(&FieldType::Author);
        let a2 = other.fields.get(&FieldType::Author);
        a1.partial_cmp(&a2)
    }
}

impl Ord for Reference {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a1 = self.fields.get(&FieldType::Author);
        let a2 = other.fields.get(&FieldType::Author);
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

// TODO? remove these functions
// pub fn _search_references<'a>(
//     references: &'a Vec<Reference>,
//     search_string: &'a String,
// ) -> Vec<&'a Reference> {
//     references
//         .iter()
//         .filter(|reference| _contains_string(reference, search_string))
//         .collect()
// }

// fn _contains_string(reference: &Reference, string: &String) -> bool {
//     if let Some(key) = reference.fields.get(&FieldType::Key) {
//         key.contains(string)
//             || reference
//                 .fields
//                 .values()
//                 .any(|value| value.contains(string))
//     } else {
//         false
//     }
// }

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
