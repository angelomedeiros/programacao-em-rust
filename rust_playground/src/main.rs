use sort::bubble_sort::bubble_sort;

mod sort {
    pub mod bubble_sort;
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Array original: {:?}", array);

    bubble_sort(&mut array);
    println!("Array ordenado: {:?}", array);
}
