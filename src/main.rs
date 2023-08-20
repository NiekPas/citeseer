use tabled::Table;

mod reference;

fn main() {
    let refs = reference::example_references();

    let table = Table::new(refs).to_string();
    println!("{}", table);
}
