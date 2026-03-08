use rand::Rng;
use std::time::Instant;

fn main() {
    let size = 10000;
    let mut rng = rand::thread_rng();
    
    let mut rnd_vec: Vec<i32> = (0..size).map(|_| rng.gen_range(1..=10000)).collect();
    println!("До сортування selection_sort: {:?}", rnd_vec);
    let start = Instant::now();
    selection_sort(&mut rnd_vec);
    let duration = start.elapsed();
    
    println!("Після сортування selection_sort: {:?}", rnd_vec);
    println!("Default format: {:?}", duration.as_micros());

    
    rnd_vec = (0..size).map(|_| rng.gen_range(1..=10000)).collect();
    println!("До сортування: {:?}", rnd_vec);
    let start = Instant::now();
    shell_sort(&mut rnd_vec);
    let duration = start.elapsed();
    println!("Після сортування shell_sort: {:?}", rnd_vec);
    println!("Default format: {:?}", duration.as_micros());
    
}

fn selection_sort<T: Ord>(arr: &mut [T]) {
    let mut c = 0;
    let mut m = 0;
    let len = arr.len();
    for i in 0..len - 1 {
        let mut j = i;
        for k in i + 1..len {
            if arr[k] < arr[j] {
                j = k;
            }
            c+=1;
        }
    
        arr.swap(i, j);
        m+=1;
    }
    println!{"Comparisons: {}, Swaps:  {}", c, m };
}

fn shell_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut c = 0;
    let mut m = 0;
    let len = arr.len();
    let mut gap = 1;
    while gap < len / 3 {
        gap = 3 * gap + 1;
    }
    while gap >= 1 {
        for i in gap..len {
            let mut j = i;
            let key = arr[j];
            while j >= gap {
                c+=1;
                if arr[j - gap] > key {
                    
                    arr[j] = arr[j - gap];
                    m+=1;
                    j -= gap; 
                }else{
                    break;
                }
                
                 
                
            }
            arr[j] = key;
            m+=1;
        }
        gap /= 3;
    }
    println!{"Comparisons: {}, Swaps:  {}", c, m };
}
