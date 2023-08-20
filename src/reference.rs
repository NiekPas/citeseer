use tabled::Tabled;

#[derive(Tabled)]
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
}

pub fn example_references() -> [Reference; 2] {
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
