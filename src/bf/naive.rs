extern crate num;

use bf::code::Code;

pub struct Runtime<'s, D> {
    code: Vec<Code<'s>>,
    data: Vec<D>,
    ptr: usize,
    input_buffer: Vec<char>,
}

impl<'s, D: num::Num + PartialOrd + Copy> Runtime<'s, D> {
    pub fn new() -> Runtime<'s, D> {
        Runtime {
            code: Vec::new(),
            data: Vec::new(),
            ptr: 0,
            input_buffer: Vec::new(),
        }
    }

    pub fn get_ptr(&self) -> usize {
        self.ptr
    }

    pub fn get_cell(&self, i: usize) -> D {
        if i < self.data.len() {
            self.data[i]
        } else {
            D::zero()
        }
    }

    pub fn set_cell(&mut self, i: usize, value: D) {
        if i >= self.data.len() {
            self.data.resize(i, D::zero());
        }
        self.data[i] = value;
    }

    pub fn add_code(&mut self, code: &Vec<Code<'s>>) {
        self.code.append(&mut code.clone());
    }

    pub fn queue_input_str(&mut self, input: &str) {
        for c in input.chars() {
            self.input_buffer.push(c);
        }
    }
}
