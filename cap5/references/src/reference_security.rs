pub fn reference_security() {
    let r;

    {
        let x = 1;
        r = &x;
        assert_eq!(*r, 1);
    }

    // assert_eq!(*r, 1); // ruim: lê a memoria que x ocupava
    static mut WORTH_POINTING_AT: &i32 = &666;

    unsafe {
        f(&WORTH_POINTING_AT);
    }

    unsafe {
        println!("STASH: {}", STASH);
    }
}

static mut STASH: &i32 = &123;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
