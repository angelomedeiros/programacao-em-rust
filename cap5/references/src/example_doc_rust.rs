pub fn example_doc_rust() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    let s = String::from("hello world");
    // let hello = &s[0..5];
    // let world = &s[6..];

    let word = first_word(&s[0..6]);
    println!("{}", &s[..]);

    // s.push('!'); // error! mesmo se s for mut

    println!("the first word is: {}", word);
}
