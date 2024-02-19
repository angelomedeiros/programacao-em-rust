pub fn reference_security() {
    let r;

    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }

    // assert_eq!(*r, 1); // ruim: lê a memoria que x ocupava
}
