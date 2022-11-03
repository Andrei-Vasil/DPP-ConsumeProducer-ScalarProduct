use std::collections::VecDeque;

pub struct Buffer {
    pub cap: usize,
    pub contents: VecDeque<i32>
}
