mod buffer;
mod generate;

use buffer::Buffer;
use generate::{generate_size, generate_vector};

fn main() {
    let size: usize = generate_size();
    let mut v1: Vec<i32> = generate_vector(size);
    let mut v2: Vec<i32> = generate_vector(size);

    println!("{:?}", v1);
    println!("{:?}", v2);

}
