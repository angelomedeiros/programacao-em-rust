use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

pub fn references_to_values() {
    let mut table = HashMap::new();
    table.insert(
        "Angelo".to_string(),
        vec!["Medeiros".to_string(), "Nóbrega".to_string()],
    );
    table.insert(
        "Maria".to_string(),
        vec![
            "Gabriella".to_string(),
            "Abreu".to_string(),
            "Lacerda".to_string(),
        ],
    );

    // show(table);
    assert_eq!(table["Angelo"][0], "Medeiros");
}

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
