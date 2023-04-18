use rand::{seq::SliceRandom, Rng};
fn main() { let mut arr: Vec<usize> = Vec::new();

    let mut rng = rand::thread_rng();

    let n = rng.gen_range(0..30);

    for i in 0..n {
        arr.push(i);
        arr.shuffle(&mut rng)
    }

    let mut arr_copy = arr.clone();

    let sorted = selection_sort(arr);

    arr_copy.sort();

    println!("{}",sorted==arr_copy);
}

fn selection_sort(mut arr: Vec<usize>) -> Vec<usize> {
    for j in 0..arr.len() {
        let mut min_ind = j;
        for i in j + 1..arr.len() {
            if arr[min_ind] > arr[i] {
                min_ind = i
            }
        }

        arr.swap(j, min_ind);
    }
    arr
}
