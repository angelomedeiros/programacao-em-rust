pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    // Percorre todos os elementos do array
    for i in 0..n {
        for j in 0..(n - 1 - i) {
            // Compara elementos adjacentes e troca-os se estiverem na ordem errada
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}
