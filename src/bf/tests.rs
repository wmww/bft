use super::*;

struct TestCase {
    code: &'static str,
    initial_data: Vec<u8>,
    initial_ptr: usize,
    input: &'static str,
    expected_data: Vec<u8>,
    expected_ptr: usize,
    expected_output: &'static str,
}

impl TestCase {
    fn new() -> TestCase {
        TestCase {
            code: "",
            initial_data: vec![],
            initial_ptr: 0,
            input: "",
            expected_data: vec![],
            expected_ptr: 0,
            expected_output: "",
        }
    }

    fn run(self) {
        let source = ::source::File::new(self.code.to_string());
        let mut runtime = debug::Runtime::<u8>::new();
        for i in 0..self.initial_data.len() {
            runtime.set_cell(i, self.initial_data[i]);
        }
        runtime.set_ptr(self.initial_ptr);
        runtime.queue_input_str(self.input);
        let mut s = ::source::span::Generator::new(&source);
        let mut tokens = ::source::lex(&source);
        // let tokens = code.iter().map(|op| op.token(s.span(0))).collect();
        runtime.add_tokens(&tokens);
        let mut result_output = String::new();
        assert_eq!(
            runtime.run(None, &mut |c| result_output.push(c)),
            Abort::Completed
        );
        assert_eq!(self.expected_output, result_output);
        let result_ptr = runtime.get_ptr();
        assert_eq!(self.expected_ptr, result_ptr);
        let mut result_data = Vec::new();
        for i in 0..self.expected_data.len() {
            result_data.push(runtime.get_cell(i));
        }
        assert_eq!(self.expected_data, result_data);
    }
}

#[test]
fn construct_runtime() {
    let mut test = TestCase::new();

    test.run();
}

#[test]
fn test_system_empty_code() {
    let mut test = TestCase::new();

    test.initial_data = vec![2, 4, 0, 62];
    test.initial_ptr = 2;
    test.expected_data = vec![2, 4, 0, 62];
    test.expected_ptr = 2;

    test.run();
}

#[test]
fn op_plus() {
    let mut test = TestCase::new();

    test.code = "+";
    test.expected_data = vec![1];

    test.run();
}

#[test]
fn op_minus() {
    let mut test = TestCase::new();

    test.code = "-";
    test.initial_data = vec![1];
    test.expected_data = vec![0];

    test.run();
}

#[test]
fn op_right() {
    let mut test = TestCase::new();

    test.code = ">";
    test.expected_ptr = 1;

    test.run();
}

#[test]
fn op_left() {
    let mut test = TestCase::new();

    test.code = "<";
    test.initial_ptr = 1;
    test.expected_ptr = 0;

    test.run();
}

#[test]
fn op_output() {
    let mut test = TestCase::new();

    test.code = ".";
    test.initial_data = vec![97];
    test.expected_output = "a";

    test.run();
}

#[test]
fn op_input() {
    let mut test = TestCase::new();

    test.code = ",";
    test.input = "a";
    test.expected_data = vec![97];

    test.run();
}

#[test]
fn move_and_chage() {
    let mut test = TestCase::new();

    test.code = "+>>+++>+<-";
    test.expected_data = vec![1, 0, 2, 1];
    test.expected_ptr = 2;

    test.run();
}
