use sort::{bubble_sort::bubble_sort, selection_sort::selection_sort};

pub mod sort;

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Array original: {:?}", array);

    bubble_sort(&mut array);
    println!("Array ordenado pelo bubble sort: {:?}", array);

    let sorted_by_selection = selection_sort(&mut array);
    println!(
        "Array ordenado pelo selection sort: {:?}",
        sorted_by_selection
    );
}
