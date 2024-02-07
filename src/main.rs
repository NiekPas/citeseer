// use std::path::PathBuf;

use core::panic;
use std::{fs, path::PathBuf};

use crate::parse::parse_bibtex;

extern crate shellexpand;

mod parse;
mod reference;

fn main() {
    println!();
    println!();
    // let home = std::env::var("HOME").unwrap();
    let path_str = "./test_bibliography.bib";
    if let Ok(bibtex_string) = fs::read_to_string(path_str) {
        if let Ok(references) = parse_bibtex(bibtex_string) {
            println!("we good");
            println!("{}", references.len());
        }
    } else {
        panic!("Oh fuck, the reading of the fucking file went wrong.")
    }
    // let references = reference::_example_references();

    // for reference in references.iter() {
    //     println!("found result: {}", reference);
    // }

    // println!("refs: {:?}", references);

    // let table = Table::new(references).to_string();
    // println!("{}", table);
}
