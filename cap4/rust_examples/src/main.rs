fn main() {
    // let name = "angelo"; // &str
    // let name = name.to_string(); // String
    // let name = name.as_str(); // &str

    // println!("{}", name.replace("a", "A")); // String

    // let mut s = "angelo".to_string(); // String
    // s.push_str(" medeiros"); // String

    // println!("{}", s);

    // let mut s = vec!["joao".to_string(), "maria".to_string(), "jose".to_string()];

    // if s[0].len() == 3 {
    //     dbg(s);
    //     s = [].to_vec(); // caso nao seja "inicializado" novamente ocorrera erro depois do if
    // } else {
    //     dbg(s);
    //     s = [].to_vec(); // caso nao seja "inicializado" novamente ocorrera erro depois do if
    // }

    // dbg(s); // nao aprensenta erro pois s foi inicializado novamente dentro do if

    // let v = vec!["joao".to_string(), "maria".to_string(), "jose".to_string()];

    // for mut i in v {
    //     i.push('!');
    //     println!("{}", i);
    // }

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

fn dbg<T: std::fmt::Debug>(s: Vec<T>) {
    println!("==== Value ====");
    dbg!(s);
    println!("==== End ====");
}

fn play_with_strings() {
    let noodles = "noodles".to_string(); // String
    let oodles = &noodles[1..]; // &str
    let poodles = "ಠ_ಠ".to_string(); // String

    println!("{}", poodles);
    println!("{}", oodles);
}
