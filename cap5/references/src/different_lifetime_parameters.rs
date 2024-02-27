use std::vec;

struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

pub fn different_lifetime_params() {
    let x = 10;
    let r;

    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }

    println!("{}", r);

    let v = vec![1, 2];
    let r = &v;
    v[0];
    let aside = v;

    let mut wave = Vec::new();
    let head = vec![0, 1];
    let tail = vec![0, 2];
    let ss = &wave;

    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    // extend(&mut wave, &mut wave);

    assert_eq!(&wave, &vec![0, 1, 0, 2]);
    dbg!(wave);
}

fn extend(vec: &mut Vec<i32>, slice: &[i32]) {
    for elt in slice {
        vec.push(*elt);
    }
}
