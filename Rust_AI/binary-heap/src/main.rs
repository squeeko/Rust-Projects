/*
Here, a `BinaryHeap` is utilized to store user ratings. By default, it acts as a
max-heap, allowing us to process the ratings in descending order. This
could be particularly useful in scenarios where we need to prioritize data
based on certain criteria.
*/

use std::collections::BinaryHeap;

fn main() {
    let mut ratings: BinaryHeap<u8> = BinaryHeap::new();
    ratings.push(3);
    ratings.push(5);
    ratings.push(1);

    while let Some(rating) = ratings.pop() {
        println!("Processing rating: {}", rating);
    }
}
