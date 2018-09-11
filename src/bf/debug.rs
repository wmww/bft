extern crate num_traits;

use self::num_traits::*;

use bf::op::Op;
use source::Span;
use source::Token;

pub struct Runtime<'s, D> {
    code: Vec<(Op, Span<'s>)>,
    stack: Vec<usize>,
    data: Vec<D>,
    ptr: usize,
    input_buffer: Vec<char>,
}

impl<'s, D: 'static + Num + NumOps + PartialOrd + Clone + Copy> Runtime<'s, D> {
    pub fn new() -> Runtime<'s, D> {
        Runtime {
            code: Vec::new(),
            stack: Vec::new(),
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

    pub fn add_tokens(&mut self, tokens: &Vec<Token<'s>>) {
        let prev_end = self.code.len();
        self.code
            .extend(tokens.iter().filter_map(|token| match token {
                Token::Bf { op, span } => Some((op.clone(), span.clone())),
                _ => None,
            }));
        if self.stack.is_empty() && prev_end < self.code.len() {
            self.stack.push(prev_end);
        }
    }

    pub fn queue_input_str(&mut self, input: &str) {
        for c in input.chars() {
            self.input_buffer.push(c);
        }
    }

    pub fn run<F>(&mut self, mut instr_cap: Option<usize>, handle_output: &F)
    where
        F: Fn(char),
    {

    }
}
