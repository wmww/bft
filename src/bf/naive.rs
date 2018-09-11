extern crate num_traits;

use self::num_traits::*;

use bf::code::Code;
use bf::ops::Op;
use source;

struct CodeIter<'s> {
    should_loop: Box<Fn() -> bool>,
    indices: Vec<::std::slice::Iter<'s, Code<'s>>>,
}

impl<'s> CodeIter<'s> {
    fn is_at_end(&self) -> bool {
        self.indices.is_empty()
    }
}

impl<'s> Iterator for CodeIter<'s> {
    type Item = (Option<Op>, &'s source::Span<'s>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.indices.len() {
                0 => { return None; }
                n => {
                    let mut i = &self.indices[n - 1];
                    match i.clone().next() {
                        None => {
                            self.indices.pop();
                        }
                        Some(Code::Op(op, span)) => {
                            i.next();
                            return Some((Some(op.clone()), span))
                        }
                        Some(Code::Loop(body, span)) => {
                            if (*self.should_loop)() {
                                self.indices.push(body.iter());
                            } else {
                                i.next();
                            }
                            return Some((None, span));
                        }
                    }
                }
            }
        }
    }
}

pub struct Runtime<'s, D> {
    code: Vec<Code<'s>>,
    iter: CodeIter<'s>,
    data: Vec<D>,
    ptr: usize,
    input_buffer: Vec<char>,
}

impl<'s, D: 'static + Num + NumOps + PartialOrd + Clone + Copy> Runtime<'s, D> {
    pub fn new() -> Runtime<'s, D> {
        let mut runtime = Runtime {
            code: Vec::new(),
            iter: CodeIter{should_loop: Box::new(|| false), indices: Vec::new()},
            data: Vec::new(),
            ptr: 0,
            input_buffer: Vec::new(),
        };
        runtime.iter.should_loop = Box::new(|| runtime.get_cell(runtime.get_ptr()) != D::zero());
        runtime
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
        let prev_end = self.code.len();
        self.code.append(&mut code.clone());
        if self.iter.is_at_end() && prev_end < self.code.len() {
            self.iter.indices.push(self.code[prev_end..].iter());
        }
    }

    pub fn queue_input_str(&mut self, input: &str) {
        for c in input.chars() {
            self.input_buffer.push(c);
        }
    }

    pub fn run_next_instr<F>(&mut self, handle_output: &F) -> bool where F: Fn(char) {
        false
    }

    pub fn run<F>(&mut self, mut instr_cap: Option<usize>, handle_output: &F) where F: Fn(char) {
        while self.run_next_instr(handle_output) {}
    }
}
