fn main() {
    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    let s = vec!["joao".to_string(), "maria".to_string(), "jose".to_string()];
    let t = s;

    /*
       let u = s, provoca um erro. Pois na maioria das atribuicoes é movido o valor da origem para
       o destino. Ou seja, o valor de s foi movido para t, deixando s não inicializada.
    */
    // let u = s;
}

fn debugger<T: std::fmt::Debug>(s: Vec<T>) {
    println!("==== Value ====");
    dbg!(s);
    println!("==== End ====");
}
