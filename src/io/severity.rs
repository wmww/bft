#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Severity {
    Debug,
    //InternalWarning,
    InternalError,
    //Warning,
    Error,
    RuntimeError,
}
