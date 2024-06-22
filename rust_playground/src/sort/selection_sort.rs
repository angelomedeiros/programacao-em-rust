pub fn selection_sort(arr: &mut [i32]) -> Vec<i32> {
    let mut novo_arr: Vec<i32> = Vec::new();

    for _ in 0..arr.len() {
        let menor = busca_menor(arr);
        novo_arr.push(arr[menor]);
        arr.copy_within(menor + 1.., menor);
    }

    return novo_arr;
}

fn busca_menor(arr: &[i32]) -> usize {
    let mut menor = arr[0];
    let mut menor_indice = 0;

    for i in 1..arr.len() {
        if arr[i] <= menor {
            menor = arr[i];
            menor_indice = i;
        }
    }

    return menor_indice;
}

#[test]
fn test_selection_sort() {
    let mut arr = [64, 55, 25, 12, 13, 9, 90, 9];
    let arr_sorted = selection_sort(&mut arr);
    println!("Array ordenado pelo selection sort: {:?}", arr_sorted);
    assert_eq!(arr_sorted, [9, 9, 12, 13, 25, 55, 64, 90]);
}

#[test]
fn test_buscar_menor() {
    let mut arr = [64, 55, 25, 12, 13, 9, 90, 9];
    let menor = busca_menor(&mut arr);
    assert_eq!(menor, 7);
}
