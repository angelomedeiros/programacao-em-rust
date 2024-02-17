use std::mem::replace;

fn main() {
    let mut s = vec!["joao".to_string(), "maria".to_string(), "jose".to_string()];

    if s[0].len() == 3 {
        dbg(s);
        s = [].to_vec(); // caso nao seja "inicializado" novamente ocorrera erro depois do if
    } else {
        dbg(s);
        s = [].to_vec(); // caso nao seja "inicializado" novamente ocorrera erro depois do if
    }

    dbg(s); // nao aprensenta erro pois s foi inicializado novamente dentro do if

    let v = vec!["joao".to_string(), "maria".to_string(), "jose".to_string()];

    for mut i in v {
        i.push('!');
        println!("{}", i);
    }

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Angelo Medeiros".to_string()),
        birth: 1995,
    });

    // let first_name = composers[0].name; //cannot move out of index of `Vec<Person>`
    let first_name = replace(&mut composers[0].name, None);
    // poderia ser usado composers[0].name.take() na linha de cima
    // ex: let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Angelo Medeiros".to_string()));

    let none_name = composers[0].name.take();
    assert_eq!(none_name, None);
}

fn dbg<T: std::fmt::Debug>(s: Vec<T>) {
    println!("==== Value ====");
    dbg!(s);
    println!("==== End ====");
}
