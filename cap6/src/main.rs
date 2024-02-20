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

    //
}
