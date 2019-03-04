extern crate num_traits;

use std::char;

use self::num_traits::*;

use super::*;
use io;
use io::Issue;
use source::Span;

pub struct Runtime<D> {
    code: Vec<(Op, Span)>,
    stack: Vec<usize>,
    data: Vec<D>,
    ptr: usize,
    input_buffer: Vec<char>,
}

enum InstrResult {
    None,
    Output(char),
    Abort(Abort),
}

impl InstrResult {
    fn abort(message: &str) -> InstrResult {
        InstrResult::Abort(Abort::Error(Issue::new(io::RuntimeError, message)))
    }
}

impl<
        D: 'static
            + Num
            + NumOps
            + WrappingAdd
            + WrappingSub
            + ToPrimitive
            + FromPrimitive
            + PartialOrd
            + Clone
            + Copy,
    > Runtime<D>
{
    pub fn new() -> Runtime<D> {
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

    pub fn set_ptr(&mut self, ptr: usize) {
        self.ptr = ptr;
    }

    pub fn get_cell(&self, i: usize) -> D {
        if i < self.data.len() {
            self.data[i]
        } else {
            D::zero()
        }
    }

    pub fn set_cell(&mut self, index: usize, value: D) {
        if index >= self.data.len() {
            self.data.resize(index + 1, D::zero());
        }
        self.data[index] = value;
    }

    pub fn add_ops(&mut self, ops: Vec<(Op, Span)>) {
        let prev_end = self.code.len();
        self.code.extend(ops);
        if self.stack.is_empty() && prev_end < self.code.len() {
            self.stack.push(prev_end);
        }
    }

    pub fn add_code(&mut self, source: &CodeSource) {
        // TODO: We shouldn't need to clone the span
        self.add_ops(
            source
                .get_code()
                .iter()
                .map(|spanned| (spanned.v, spanned.s.clone()))
                .collect(),
        );
    }

    pub fn queue_input_str(&mut self, input: &str) {
        for c in input.chars().rev() {
            self.input_buffer.push(c);
        }
    }

    fn run_instr(&mut self, instr: usize) -> InstrResult {
        let ptr = self.ptr;
        let op = self.code[instr].0;
        match op {
            Op::Plus => {
                let value = self.get_cell(ptr).wrapping_add(&D::one());
                self.set_cell(ptr, value);
                InstrResult::None
            }
            Op::Minus => {
                let value = self.get_cell(ptr).wrapping_sub(&D::one());
                self.set_cell(ptr, value);
                InstrResult::None
            }
            Op::Left => {
                if self.ptr == 0 {
                    InstrResult::abort("Pointer moved left of the starting point")
                } else {
                    self.ptr -= 1;
                    InstrResult::None
                }
            }
            Op::Right => {
                self.ptr += 1;
                InstrResult::None
            }
            Op::Output => InstrResult::Output(
                char::from_u32(self.get_cell(self.ptr).to_u32().unwrap()).unwrap_or('\0'),
            ),
            Op::Input => match self.input_buffer.pop() {
                Some(c) => {
                    let value = D::from_u8(c as u8).unwrap();
                    self.set_cell(ptr, value);
                    InstrResult::None
                }
                None => InstrResult::Abort(Abort::AwaitingInput),
            },
            Op::Start => {
                if self.get_cell(self.ptr) == D::zero() {
                    let mut instr = instr + 1;
                    let mut level = 1;
                    loop {
                        if instr >= self.code.len() {
                            break InstrResult::Abort(Abort::Completed);
                        }
                        match self.code[instr].0 {
                            Op::Start => level += 1,
                            Op::End => level -= 1,
                            _ => (),
                        }
                        if level <= 0 {
                            let last_index = self.stack.len() - 1;
                            self.stack[last_index] = instr;
                            break InstrResult::None;
                        }
                        instr += 1;
                    }
                } else {
                    self.stack.push(instr);
                    InstrResult::None
                }
            }
            Op::End => {
                if self.stack.len() <= 1 {
                    InstrResult::abort("Extraneous closing brace")
                } else {
                    if self.get_cell(self.ptr) == D::zero() {
                        self.stack.pop().unwrap();
                        let last_index = self.stack.len() - 1;
                        self.stack[last_index] = instr;
                    } else {
                        let last_index = self.stack.len() - 1;
                        self.stack[last_index] = self.stack[last_index - 1];
                    }
                    InstrResult::None
                }
            }
        }
    }

    pub fn run<F>(&mut self, mut instr_cap: Option<usize>, handle_output: &mut F) -> Abort
    where
        F: FnMut(char),
    {
        loop {
            let instr = *self.stack.last().unwrap();
            if instr >= self.code.len() {
                break Abort::Completed;
            }
            match self.run_instr(instr) {
                InstrResult::None => (),
                InstrResult::Output(c) => handle_output(c),
                InstrResult::Abort(a) => break a,
            }
            let last_index = self.stack.len() - 1;
            self.stack[last_index] += 1;
            if let Some(instr_cap) = &mut instr_cap {
                *instr_cap -= 1;
                if *instr_cap <= 0 {
                    break Abort::InstrCapped;
                }
            }
        }
    }
}
