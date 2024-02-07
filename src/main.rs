use crate::reference::CsvError::{ParseError, ReadError};
use std::path::PathBuf;

use tabled::Table;
extern crate shellexpand;

mod reference;

fn main() {
    println!();
    println!();
    let home = std::env::var("HOME").unwrap();
    let path_str = format!("{}{}", home, "/.citeseer/references.csv");
    let references_path = PathBuf::from(path_str);
    let refs = reference::references_from_csv(&references_path);

    match refs {
        Ok(references) => {
            // for reference in references.iter() {
            //     println!("found result: {}", reference);
            // }

            println!("refs: {:?}", references);

            let table = Table::new(references).to_string();
            println!("{}", table);
        }

        Err(ReadError(err)) | Err(ParseError(err)) => {
            println!("Failed to parse csv file: {}", err);
        }
    }
}
