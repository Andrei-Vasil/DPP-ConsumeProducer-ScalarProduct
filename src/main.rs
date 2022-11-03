mod buffer;
mod generate;

use buffer::Buffer;
use generate::{generate_size, generate_vector};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn producer(v1: Arc<Mutex<Vec<i32>>>, v2: Arc<Mutex<Vec<i32>>>, buffer: Arc<Mutex<Buffer>>, cond_var: Arc<(Mutex<bool>, Condvar)>) {
    
}

fn consumer(v1: Arc<Mutex<Vec<i32>>>, v2: Arc<Mutex<Vec<i32>>>, buffer: Arc<Mutex<Buffer>>, cond_var: Arc<(Mutex<bool>, Condvar)>) {
    
}

fn main() {
    let size: usize = generate_size();
    let v1: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(generate_vector(size)));
    let v2: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(generate_vector(size)));
    let buffer: Arc<Mutex<Buffer>> = Arc::new(Mutex::new(Buffer{cap: 25, contents: vec![]}));
    let cond_var: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));

    let v1_copy = Arc::clone(&v1);
    let v2_copy = Arc::clone(&v2);
    let buffer_copy = Arc::clone(&buffer);
    let cond_var_copy = Arc::clone(&cond_var);
    let producer_handle = thread::spawn(move || {
        producer(v1_copy, v2_copy, buffer_copy, cond_var_copy);
    });

    let v1_copy = Arc::clone(&v1);
    let v2_copy = Arc::clone(&v2);
    let buffer_copy = Arc::clone(&buffer);
    let cond_var_copy = Arc::clone(&cond_var);
    let consumer_handle = thread::spawn(move || {
        consumer(v1_copy, v2_copy, buffer_copy, cond_var_copy);
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
