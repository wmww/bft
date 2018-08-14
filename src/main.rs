mod options;

pub enum Priority {
    Debug,
    InternalWarning,
    InternalError,
    Warning,
    Error,
}

pub fn message(priority: Priority, msg: &str) {
    println!(
        "{}: {}",
        match priority {
            Priority::Debug => "Debug",
            Priority::InternalWarning => "Internal warning",
            Priority::InternalError => "Internal error",
            Priority::Warning => "Warning",
            Priority::Error => "Error",
        },
        msg
    );
}

pub fn debug(msg: &str) {
    message(Priority::Debug, msg);
}

fn main() {
    let options = options::Options::new_default().with_cmd_line();
    if let Some(s) = options.filepath {
        debug(&format!("Loading source code: {}", s));
    }
}
