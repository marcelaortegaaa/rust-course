// From Rust docs
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    let push_vec = vec![3, 4, 5, 6, 2, 4, 5, 3];

    heap.push(1);
    heap.push(5);
    heap.push(2);

    heap.extend(push_vec);

    // Peek shows the most important item in the heap
    let peek = heap.peek();
    println!("{peek:?}");

    // Items are returned in a random order
    // for x in &heap {
    //     println!("{x}");
    // }

    // while let Some(item) = heap.pop() {
    //     println!("{item}");
    // }

    let sorted_vec = heap.into_sorted_vec();
    println!("{sorted_vec:?}");
}
