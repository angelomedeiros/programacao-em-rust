fn main() {
    // #[derive(Debug)]
    // struct Person {
    //     name: Option<String>,
    //     birth: i32,
    // }

    // let mut composers = Vec::new();
    // composers.push(Person {
    //     name: Some("Angelo Medeiros".to_string()),
    //     birth: 1995,
    // });

    // // let first_name = composers[0].name; //cannot move out of index of `Vec<Person>`
    // let first_name = replace(&mut composers[0].name, None);
    // // poderia ser usado composers[0].name.take() na linha de cima
    // // ex: let first_name = composers[0].name.take();
    // assert_eq!(first_name, Some("Angelo Medeiros".to_string()));

    // let none_name = composers[0].name.take();
    // assert_eq!(none_name, None);

    // println!(
    //     r##"
    //         Teste "raw string"
    //     "##
    // );

    // let method = b"GET"; // &[u8; 3]
    // println!("{:?}", method);

    play_with_strings();
}

fn play_with_strings() {
    let noodles = "noodles".to_string(); // String
    let oodles = &noodles[1..]; // &str
    let poodles = "ಠ_ಠ".to_string(); // String

    println!("{}", poodles);
    println!("{}", oodles);
}
