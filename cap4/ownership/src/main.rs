use std::rc::Rc;

struct Person {
    age: i32,
    name: Option<String>,
}

#[derive(Copy, Clone)]
struct Car {
    max_vel: i16,
    is_eletric: bool,
}

fn main() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    let s = vec!["joao".to_string(), "maria".to_string(), "jose".to_string()];
    let t = s;

    /*
       let u = s, provoca um erro. Pois na maioria das atribuicoes é movido o valor da origem para
       o destino. Ou seja, o valor de s foi movido para t, deixando s não inicializada. Agora t
       possui o valor de s
    */
    // let u = s;

    /*
       A solucao para obter copias é que devem ser solicitads explicitamente. Dessa maneira terá o
       mesmo comportamento que teria no C++
    */
    let t = t.clone();
    let u = t.clone(); // Nao apresenta mais erro, pois u agora possui uma copia de t

    // ====== Mais operacoes que movem ======
    let mut ss = "angelo".to_string();
    ss = "Angelo".to_string(); // "angelo" é dropado aqui

    println!("{}", ss);

    let mut tt = "Bruce".to_string();
    let uu = tt;
    tt = "Batman".to_string(); // Nada é dropado aqui pois uu tomou posso de tt, tornado tt nao incializada

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    // let third = v[2]; // erro se tentar retirar o valor v[2]
    // let fifth = v[4]; // erro se tentar retirar o valor v[4]

    let fifth = v.pop().expect("vector empty"); // remove 105

    println!("fifth: {}", fifth);
    dbg!(&v);

    let second = v.swap_remove(1);
    println!("second: {}", second);

    dbg!(&v);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());

    assert_eq!("103", third);

    dbg!(&v);

    let v = vec!["a".to_string(), "b".to_string()];

    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let mut composers = Vec::new();
    let person1 = Person {
        age: 28,
        name: Some("Angelo".to_string()),
    };

    composers.push(person1);

    let first_name = std::mem::replace(&mut composers[0].name, None);
    let fisrt_age = composers[0].age;

    dbg!(first_name.expect("empty value"));
    dbg!(fisrt_age);

    let car = Car {
        max_vel: 250,
        is_eletric: true,
    };

    print_car(car);
    /*
       Na chamada abaixo de car.max_val se o struct nao tiver marcado como #[derive(Copy, Clone)]
       vai dar erro, pois um struct por padrao nao tipo copy, mas add #[derive(Copy, Clone)], e
       isso só é possível se todos os valores forem do tipo copy, que é esse o caso pois todos os
       tipos do struct (i16, bool) são do tipo copy
    */
    println!("{}", car.max_vel);

    let s = Rc::new("Angelo".to_string());
    let t = s.clone();
    let u = s.clone();

    println!("s: {}", s);
    println!("t: {}", t);
    println!("u: {}", u);
}

fn print_car(car: Car) {
    println!("Max vel: {}km/h", car.max_vel);
    println!("Is eletric: {}", car.is_eletric);
}
