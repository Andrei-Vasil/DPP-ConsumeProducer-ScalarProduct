use rand::Rng;

pub fn generate_size() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..50);
}

pub fn generate_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<i32> = vec![];
    for _ in 0..size {
        // v.push(rng.gen_range(0..500));
        v.push(2);
    }
    return v;
}
