fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    // Percorre todos os elementos do array
    for i in 0..n {
        for j in 0..(n - 1 - i) {
            // Compara elementos adjacentes e troca-os se estiverem na ordem errada
            if arr[j] > arr[j + 1] {
                // Chama nossa própria função de troca
                swap(arr, j, j + 1);
            }
        }
    }
}

fn swap(arr: &mut [i32], a: usize, b: usize) {
    let temp = arr[a];
    arr[a] = arr[b];
    arr[b] = temp;
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Array original: {:?}", array);

    bubble_sort(&mut array);
    println!("Array ordenado: {:?}", array);
}
