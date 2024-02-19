pub fn ref_rust_versus_cpp() {
    let mut y = 32;
    let m = &mut y;

    *m += 32;
    assert!(*m == 64);
    // dbg!(m);
    dbg!(*m);
}
