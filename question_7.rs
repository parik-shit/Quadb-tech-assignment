use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut min_heap = BinaryHeap::new();

        for &num in arr.iter() {
            min_heap.push(Reverse(num));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        Some(min_heap.pop().unwrap().0)
    } else {
        None
    }
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
