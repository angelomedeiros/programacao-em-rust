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

#[test]
fn test_bubble_sort() {
    let mut arr = [64, 34, 25, 12, 22, 11, 90, 11];
    bubble_sort(&mut arr);
    assert_eq!(arr, [11, 11, 12, 22, 25, 34, 64, 90]);
}
