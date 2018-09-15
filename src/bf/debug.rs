extern crate num_traits;

use std::char;

use self::num_traits::*;

use bf::op::Op;
use bf::Abort;
use log::Issue;
use log::Severity;
use source::Span;
use source::Token;

pub struct Runtime<'s, D> {
    code: Vec<(Op, Span<'s>)>,
    stack: Vec<usize>,
    data: Vec<D>,
    ptr: usize,
    input_buffer: Vec<char>,
}

impl<'s, D: 'static + Num + NumOps + ToPrimitive + FromPrimitive + PartialOrd + Clone + Copy>
    Runtime<'s, D>
{
    pub fn new() -> Runtime<'s, D> {
        Runtime {
            code: Vec::new(),
            stack: vec![0],
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

    fn set_cell(&mut self, index: usize, value: D) {
        if index >= self.data.len() {
            self.data.resize(index + 1, D::zero());
        }
        self.data[index] = value;
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
        for c in input.chars().rev() {
            self.input_buffer.push(c);
        }
    }

    pub fn run<F>(&mut self, mut instr_cap: Option<usize>, handle_output: &F) -> Abort<'s>
    where
        F: Fn(char),
    {
        loop {
            let instr = *self.stack.last().unwrap();
            if instr > self.code.len() {
                break Abort::Completed;
            }
            let (op, _span) = self.code[instr].clone();
            println!("Running {}", op);
            match op {
                Op::Plus => {
                    let ptr = self.ptr;
                    let value = self.get_cell(ptr) + D::one();
                    self.set_cell(ptr, value);
                }
                Op::Minus => {
                    let ptr = self.ptr;
                    let value = self.get_cell(ptr) - D::one();
                    self.set_cell(ptr, value);
                }
                Op::Left => {
                    if self.ptr == 0 {
                        break Abort::Error(Issue::new(
                            Severity::RuntimeError,
                            "Pointer moved left of the starting point",
                        ));
                        // break Abort::Error(span.issue(Severity::RuntimeError, "Pointer moved left of the start"))
                    }
                    self.ptr -= 1;
                }
                Op::Right => self.ptr += 1,
                Op::Output => handle_output(
                    char::from_u32(self.get_cell(self.ptr).to_u32().unwrap()).unwrap_or('\0'),
                ),
                Op::Input => {
                    let ptr = self.ptr;
                    let value = match self.input_buffer.pop() {
                        Some(c) => D::from_u8(c as u8).unwrap(),
                        None => break Abort::AwaitingInput,
                    };
                    self.set_cell(ptr, value);
                }
                Op::Start => {
                    if self.get_cell(self.ptr) == D::zero() {

                    } else {
                        self.stack.push(instr);
                    }
                }
                Op::End => {
                    if self.stack.len() <= 1 {
                        break Abort::Error(Issue::new(
                            Severity::RuntimeError,
                            "Extraneous closing brace",
                        ));
                    // break Abort::Error(span.issue(Severity::RuntimeError, "Extraneous closing brace"));
                    } else if self.get_cell(self.ptr) == D::zero() {
                        self.stack.pop().unwrap();
                        let last_index = self.stack.len() - 1;
                        self.stack[last_index] = instr;
                    } else {
                        let last_index = self.stack.len() - 1;
                        self.stack[last_index] = self.stack[last_index];
                    }
                }
            }
            let last_index = self.stack.len() - 1;
            self.stack[last_index] += 1;
        }
    }
}
