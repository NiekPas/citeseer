use core::fmt;
// use std::{collections::HashMap, io::Error, io::ErrorKind, io::Read, path::Path};
use std::{collections::HashMap, io::Read, path::Path};
use tabled::Tabled;

#[derive(Tabled, Debug)]
pub struct Reference {
    pub author: String,
    pub title: String,
    pub year: String,
    pub journal: String,
    pub volume: String,
    pub number: String,
    pub pages: String,
    pub month: String,
    pub note: String,
    pub key: String,
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.author, self.year)
    }
}

impl Reference {
    // Constructor method that sets all fields to ""
    pub fn new_empty() -> Self {
        Reference {
            author: "".to_string(),
            title: "".to_string(),
            year: "".to_string(),
            journal: "".to_string(),
            volume: "".to_string(),
            number: "".to_string(),
            pages: "".to_string(),
            month: "".to_string(),
            note: "".to_string(),
            key: "".to_string(),
        }
    }

    pub fn set_field(&mut self, field: &Field, value: &String) -> &mut Reference {
        match field {
            Field::Author => {
                self.author = value.to_string();
                return self;
            }
            Field::Title => {
                self.title = value.to_string();
                return self;
            }
            Field::Year => {
                self.year = value.to_string();
                return self;
            }
            Field::Journal => {
                self.journal = value.to_string();
                return self;
            }
            Field::Volume => {
                self.volume = value.to_string();
                return self;
            }
            Field::Number => {
                self.number = value.to_string();
                return self;
            }
            Field::Pages => {
                self.pages = value.to_string();
                return self;
            }
            Field::Month => {
                self.month = value.to_string();
                return self;
            }
            Field::Note => {
                self.note = value.to_string();
                return self;
            }
            Field::Key => {
                self.key = value.to_string();
                return self;
            }
        }
    }
}

pub fn _example_references() -> [Reference; 2] {
    return [
        Reference {
            author: "Smith, John".to_string(),
            title: "The Rust Programming Language".to_string(),
            year: "2021".to_string(),
            journal: "Rust Journal".to_string(),
            volume: "5".to_string(),
            number: "2".to_string(),
            pages: "100-120".to_string(),
            month: "July".to_string(),
            note: "This is a sample reference.".to_string(),
            key: "smith2021".to_string(),
        },
        Reference {
            author: "Doe, Jane".to_string(),
            title: "Introduction to Rust".to_string(),
            year: "2022".to_string(),
            journal: "Rust Gazette".to_string(),
            volume: "7".to_string(),
            number: "1".to_string(),
            pages: "50-70".to_string(),
            month: "January".to_string(),
            note: "Another sample reference.".to_string(),
            key: "doe2022".to_string(),
        },
    ];
}

pub enum CsvError {
    ReadError(String),
    ParseError(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Field {
    Author,
    Title,
    Year,
    Journal,
    Volume,
    Number,
    Pages,
    Month,
    Note,
    Key,
}

pub fn references_from_csv(path: &Path) -> Result<Vec<Reference>, CsvError> {
    let mut file = std::fs::File::open(path).map_err(|err| CsvError::ReadError(err.to_string()))?;
    let mut file_contents = String::new();

    match file.read_to_string(&mut file_contents) {
        Ok(_bytes) => {
            let field_indices = get_field_indices(&file_contents)?;
            // TODO
            let references: Result<Vec<Reference>, CsvError> =
                parse_references(&field_indices, &file_contents);
            return references;
        }
        Err(err) => {
            return Err(CsvError::ReadError(err.to_string()));
        }
    }
}

fn get_field_indices(csv: &String) -> Result<HashMap<Field, usize>, CsvError> {
    let lines: Vec<&str> = csv.lines().collect();
    // Map fields to their location in the csv file
    let mut field_indices: HashMap<Field, usize> = HashMap::new();

    if let Some(headers) = lines.get(0) {
        let headers: Vec<&str> = headers.split(',').collect();

        for (idx, header) in headers.iter().enumerate() {
            match *header {
                "author" => field_indices.insert(Field::Author, idx),
                "title" => field_indices.insert(Field::Title, idx),
                "year" => field_indices.insert(Field::Year, idx),
                "journal" => field_indices.insert(Field::Journal, idx),
                "volume" => field_indices.insert(Field::Volume, idx),
                "number" => field_indices.insert(Field::Number, idx),
                "pages" => field_indices.insert(Field::Pages, idx),
                "month" => field_indices.insert(Field::Month, idx),
                "note" => field_indices.insert(Field::Note, idx),
                "key" => field_indices.insert(Field::Key, idx),
                _ => None, // TODO return parse error here
            };
        }

        Ok(field_indices)
    } else {
        return Err(CsvError::ParseError(
            "Could not parse headers from CSV file: file contains no lines.".to_string(),
        ));
    }
}

fn parse_references(
    field_indices: &HashMap<Field, usize>,
    file_contents: &str,
) -> Result<Vec<Reference>, CsvError> {
    let rows: Vec<&str> = file_contents.lines().skip(1).collect();
    for row in rows {
        let cells: Vec<&str> = row.split(',').collect();
        let mut currentReference = Reference::new_empty();

        for (field, index) in field_indices.iter() {
            println!("field: {:?} index: {index}", field);

            let value = cells.get(*index);

            currentReference.set_field(field, value);
        }

        // let authorIndex = field_indices.get(&Field::Author);
        // let titleIndex = field_indices.get(&Field::Title);
        // let yearIndex = field_indices.get(&Field::Year);
        // let journalIndex = field_indices.get(&Field::Journal);
        // let volumeIndex = field_indices.get(&Field::Volume);
        // let numberIndex = field_indices.get(&Field::Number);
        // let pagesIndex = field_indices.get(&Field::Pages);
        // let monthIndex = field_indices.get(&Field::Month);
        // let noteIndex = field_indices.get(&Field::Note);
        // let keyIndex = field_indices.get(&Field::Key);
    }

    todo!();

    // _returnval
}
