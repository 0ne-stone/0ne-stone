use std::collections::BinaryHeap;

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];

    for number in input {
        remainder_vec.push(*number)
    }

    remainder_vec
}

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() {
        println!("Popped of {number}. Remaining numbers are: {:?}", show_remainder(&my_heap));
    }
}
