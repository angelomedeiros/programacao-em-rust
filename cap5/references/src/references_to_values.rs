use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

pub fn references_to_values() {
    let mut table = HashMap::new();
    table.insert(
        "Angelo".to_string(),
        vec!["Nóbrega".to_string(), "Medeiros".to_string()],
    );
    table.insert(
        "Maria".to_string(),
        vec![
            "Gabriella".to_string(),
            "Abreu".to_string(),
            "Lacerda".to_string(),
        ],
    );

    show(&table);
    dbg!(&table);
    sort_works(&mut table);
    dbg!(&table);
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
