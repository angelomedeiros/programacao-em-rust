pub fn ref_rust_versus_cpp() {
    let mut y = 32;
    let m = &mut y;

    *m += 32;
    assert!(*m == 64);

    // dbg!(m);
    // assert_eq!(y, 10);
    // dbg!(*m);
    println!("Valores => m: {}, *m: {}, &m: {}", m, *m, &m);
    println!("Endereços => m: {:p}, &m: {:p}", m, &m);
}
