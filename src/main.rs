fn main() {
    println!("Hello, world!");
    let mut test = [5, 2, 9, 1, 5, 6];
    selection_sort(&mut test);
    println!("{:?}", test);
    let mut test = [5, 2, 9, 1, 5, 6];
    shell_sort(&mut test);
    println!("{:?}", test);
}

fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len - 1 {
        let mut j = i;
        for k in i + 1..len {
            if arr[k] < arr[j] {
                j = k;
            }
        }
        if j != i {
            arr.swap(i, j);
        }
    }
}

fn shell_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    let mut gap = 1;
    while gap < len / 3 {
        gap = 3 * gap + 1;
    }
    while gap >= 1 {
        for i in gap..len {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j - gap, j);
                j -= gap;
            }
        }
        gap /= 3;
    }
}
