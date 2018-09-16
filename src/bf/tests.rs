use super::*;

fn test_program(
    code: Vec<Op>,
    initial_data: Vec<u8>,
    initial_ptr: usize,
    input: &str,
    exit_data: Vec<u8>,
    exit_ptr: usize,
    output: &str,
) {
    let source = ::source::File::new(String::new());
    let mut runtime = debug::Runtime::<u8>::new();
    for i in 0..initial_data.len() {
        runtime.set_cell(i, initial_data[i]);
    }
    runtime.set_ptr(initial_ptr);
    runtime.queue_input_str(input);
    let mut s = ::source::span::Generator::new(&source);
    let mut tokens = Vec::new();
    for op in code {
        tokens.push(op.token(s.span(0)));
    }
    // let tokens = code.iter().map(|op| op.token(s.span(0))).collect();
    runtime.add_tokens(&tokens);
    let mut real_output = String::new();
    assert_eq!(
        runtime.run(None, &mut |c| real_output.push(c)),
        Abort::Completed
    );
}

#[test]
fn construct_runtime() {
    test_program(vec![], vec![], 0, "", vec![], 0, "");
}

#[test]
fn op_add() {
    test_program(vec![Op::Plus], vec![], 0, "", vec![1], 0, "");
}
