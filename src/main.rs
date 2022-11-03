mod buffer;
mod generate;

use buffer::Buffer;
use generate::{generate_size, generate_vector};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::collections::VecDeque;

fn producer(v1: Arc<Mutex<Vec<i32>>>, v2: Arc<Mutex<Vec<i32>>>, buffer: Arc<Mutex<Buffer>>, cond_var: Arc<(Mutex<i32>, Condvar)>) {
    let mut i: usize = 0;
    let guard = v1.lock().unwrap();
    let v_size: usize = guard.len();
    drop(guard);
    while i < v_size {
        let v1_guard = v1.lock().unwrap();
        let a = v1_guard[i];
        drop(v1_guard);
        
        let v2_guard = v2.lock().unwrap();
        let b = v2_guard[i];
        drop(v2_guard);

        let product = a * b;

        let mut buffer_guard = buffer.lock().unwrap();
        buffer_guard.contents.push_back(product);
        drop(buffer_guard);

        let mut guard = cond_var.0.lock().unwrap();
        *guard += 1;
        cond_var.1.notify_one();
        drop(guard);

        i += 1;
    }
}

fn consumer(v_size: usize, buffer: Arc<Mutex<Buffer>>, cond_var: Arc<(Mutex<i32>, Condvar)>) -> i32 {
    let mut scalar_product: i32 = 0;
    let mut no_of_operations: usize = 0;
    while no_of_operations < v_size {
        let mut received = cond_var.0.lock().unwrap();
        while *received == 0 {
            received = cond_var.1.wait(received).unwrap();
        }
        *received -= 1;
        drop(received);

        let mut guard = buffer.lock().unwrap();
        let product = guard.contents.pop_front().unwrap();
        drop(guard);
        scalar_product += product;
        no_of_operations += 1;
    }
    return scalar_product;
}

fn main() {
    let size: usize = generate_size();
    let v1: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(generate_vector(size)));
    let v2: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(generate_vector(size)));
    let buffer: Arc<Mutex<Buffer>> = Arc::new(Mutex::new(Buffer{contents: VecDeque::new()}));
    let cond_var: Arc<(Mutex<i32>, Condvar)> = Arc::new((Mutex::new(0), Condvar::new()));

    println!("your vector #1: {:?}", v1.lock().unwrap());
    println!("your vector #2: {:?}", v2.lock().unwrap());

    let v1_copy = Arc::clone(&v1);
    let v2_copy = Arc::clone(&v2);
    let buffer_copy = Arc::clone(&buffer);
    let cond_var_copy = Arc::clone(&cond_var);
    let producer_handle = thread::spawn(move || {
        producer(v1_copy, v2_copy, buffer_copy, cond_var_copy);
    });

    let buffer_copy = Arc::clone(&buffer);
    let cond_var_copy = Arc::clone(&cond_var);
    let consumer_handle = thread::spawn(move || {
        let scalar_product = consumer(size, buffer_copy, cond_var_copy);
        println!("your scalar product: {}", scalar_product);
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
