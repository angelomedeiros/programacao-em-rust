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

    struct Person {
        name: Option<String>,
        age: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        age: 28,
        name: Some("Angelo".to_string()),
    });

    let first_name = composers[0].name.take(); // o mesmo que replace(&mut composers[0].name, None)
    dbg!(first_name.expect("empty value"));
}
