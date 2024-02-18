use std::collections::HashMap;

pub fn references_to_values() {}

fn show(table: HashMap<String, Vec<String>>) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
