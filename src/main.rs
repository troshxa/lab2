fn main() {
    println!("Hello, world!");
    let mut test = [5, 2, 9, 1, 5, 6];
    selection_sort(&mut test);
    println!("{:?}", test);
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let Len = arr.len();
    for i in 0..Len - 1 {
        let mut j = i;
        for k in i + 1..Len {
            if arr[k] < arr[j] {
                j = k;
            }
        }
        if j != i {
            arr.swap(i, j);
        }
    }
}
