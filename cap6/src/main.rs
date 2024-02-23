fn main() {
    // Statements
    let msg = {
        let name = "angelo".to_string();

        name // return value to variable
    };

    println!("Hello, world, {}!", msg);

    // Code block
    fn show_files() {
        let names = vec!["Maria".to_string(), "Gabriel".to_string()];

        fn print_vec(vector: &Vec<String>) {
            // dbg!(&names); // Cannot access names outside of a nested function
            for v in vector {
                println!("{}", v);
            }
        }

        print_vec(&names);
    }

    show_files();

    let code = 0;

    // Switch
    let message = match code {
        0 => "Ok".to_string(),
        1 => "Wires Tangled".to_string(),
        _ => "Unkown".to_string(), // The _ is the default operator
    };

    println!("Message: {}", message);

    // Ifs
    let suggested_car = if code == 0 {
        "Fiat uno".to_string()
    } else if code == 1 {
        "BMW".to_string()
    } else {
        "Unkown".to_string()
    };

    println!("Suggested car: {}", suggested_car);

    // Loops
    for i in 0..5 {
        println!("i: {}", i); // 0, 1, ..., 4
    }

    let range = std::ops::Range { start: 0, end: 5 };

    for i in range {
        println!("i: {}", i); // 0, 1, ..., 4
    }

    // Flow control
    let range: Vec<usize> = (1..10).collect();

    'flowcontrol: for i in &range {
        for j in &range {
            if i == &(range.len()) {
                print!("END!");
                break 'flowcontrol;
            } else {
                print!("{}:{} ", i, j);
            }
        }
        print!("...\n");
    }
}
