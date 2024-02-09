use core::panic;
use std::fs;

use reference::Reference;
use tabled::{
    builder::Builder,
    settings::{
        peaker::{PriorityMax, PriorityMin},
        Settings, Width,
    },
    Table,
};

use crate::parse::parse_bibtex;

extern crate shellexpand;

mod parse;
mod reference;

fn main() {
    let path_str = "./test_bibliography_small.bib";
    if let Ok(bibtex_string) = fs::read_to_string(path_str) {
        if let Ok(references) = parse_bibtex(bibtex_string) {
            println!("{}", references.len());
            let table = test_table(references);
            println!("{}", table);
        }
    } else {
        panic!("Oh fuck, the reading of the fucking file went wrong.")
    }
}

fn test_table<'a>(references: Vec<Reference>) -> String {
    // let references: [Reference; 2] = _example_references();
    let mut builder = Builder::default();
    builder.push_record(["Title", "Author"]);

    for reference in references {
        builder.push_record(reference.as_array());
    }
    let width: usize = 80;
    let mut table = builder.build();

    table.with(Settings::new(
        Width::truncate(width).priority::<PriorityMax>(),
        Width::increase(width).priority::<PriorityMin>(),
    ));
    table.to_string()
}

fn build_table(references: Vec<Reference>) -> Table {
    let mut builder = Builder::default();
    for reference in references {
        let fields = reference.fields.values();
        builder.push_record(fields);
    }
    builder.build()
}
