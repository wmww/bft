struct Runtime<'a> {
    code: Vec<(Op, &'a source::Span)>,
    data: Vec<u8>,
}