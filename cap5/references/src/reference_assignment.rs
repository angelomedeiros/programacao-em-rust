pub fn reference_assignment() {
    let x = 10;
    let y = 20;
    let mut r = &x;

    if false {
        r = &y;
    }

    println!("Valor de r: {}", *r);

    let r = factorial(5);
    println!("Factorial 5: {}", r);
}

fn factorial(n: usize) -> usize {
    // (1..n + 1).product() // Solucao alternativa
    if n == 1 {
        return 1;
    }
    n * factorial(n - 1)
}
