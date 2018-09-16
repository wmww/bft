use super::*;

fn test_program(
    code: Vec<Op>,
    initial_data: Vec<u8>,
    initial_ptr: usize,
    input: &str,
    expected_data: Vec<u8>,
    expected_ptr: usize,
    expected_output: &str,
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
    let mut result_output = String::new();
    assert_eq!(
        runtime.run(None, &mut |c| result_output.push(c)),
        Abort::Completed
    );
    assert_eq!(expected_output, result_output);
    let result_ptr = runtime.get_ptr();
    assert_eq!(expected_ptr, result_ptr);
    let mut result_data = Vec::new();
    for i in 0..expected_data.len() {
        result_data.push(runtime.get_cell(i));
    }
    assert_eq!(expected_data, result_data);
}

#[test]
fn construct_runtime() {
    let code = vec![];
    let initial_data = vec![];
    let initial_ptr = 0;
    let input = "";
    let expected_data = vec![];
    let expected_ptr = 0;
    let expected_output = "";
    test_program(code, initial_data, initial_ptr, input, expected_data, expected_ptr, expected_output);
}

#[test]
fn test_system_empty_code() {
    let initial_data = vec![2, 4, 0, 62];
    let initial_ptr = 2;
    let expected_data = vec![2, 4, 0, 62];
    let expected_ptr = 2;
    test_program(vec![], initial_data, initial_ptr, "", expected_data, expected_ptr, "");
}

#[test]
fn op_plus() {
    let code = vec![Op::Plus];
    let expected_data = vec![1];
    test_program(code, vec![], 0, "", expected_data, 0, "");
}

#[test]
fn op_minus() {
    let code = vec![Op::Minus];
    let initial_data = vec![1];
    let expected_data = vec![0];
    test_program(code, initial_data, 0, "", expected_data, 0, "");
}
