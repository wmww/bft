pub enum Priority {
    Debug,
    //InternalWarning,
    InternalError,
    //Warning,
    //Error,
}

pub fn message(priority: Priority, msg: &str) {
    println!(
        "{}: {}",
        match priority {
            Priority::Debug => "Debug",
            //Priority::InternalWarning => "Internal warning",
            Priority::InternalError => "Internal error",
            //Priority::Warning => "Warning",
            //Priority::Error => "Error",
        },
        msg
    );
}

#[macro_export]
macro_rules! bft_error {
    ($ops:ident, $($arg:tt)*) => ({
        log::message(log::Priority::InternalError, &format!($($arg)*));
        std::process::exit(1);
    })
}

#[macro_export]
macro_rules! bft_log {
    ($ops:ident, $($arg:tt)*) => ({
        if $ops.debug {
            log::message(log::Priority::Debug, &format!($($arg)*));
        }
    })
}
