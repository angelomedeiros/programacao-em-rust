fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        // Flag para verificar se houve trocas na iteração atual
        let mut swapped = false;

        for j in 0..(n - i - 1) {
            // Compara elementos adjacentes e troca-os se estiverem na ordem errada
            if arr[j] > arr[j + 1] {
                swap(arr, j, j + 1);
                swapped = true;
            }
        }

        // Se não houve trocas, o array já está ordenado
        if !swapped {
            break;
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
