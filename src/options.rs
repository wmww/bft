use std;

extern crate clap;
use self::clap::{App, Arg};

#[derive(Debug)]
pub struct Options {
    pub filepath: Option<String>, // code to run
    pub fixup_file: bool,         // if to automatically fix problems found in the file
    pub debug: bool,              // if to run bft in debug mode
}

impl Options {
    pub fn new_default() -> Options {
        Options {
            filepath: None,
            fixup_file: true,
            debug: false,
        }
    }

    pub fn with_cmd_line(self) -> Options {
        let mut app = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .arg(
                Arg::with_name("FILEPATH")
                    .help("The input brainfuck source code")
                    .index(1),
            )
            .arg(
                Arg::with_name("DEBUG")
                    .short("d")
                    .long("debug")
                    .help("Enable debug mode"),
            )
            .arg(
                Arg::with_name("READONLY")
                    .short("r")
                    .long("readonly")
                    .help("Don't apply fixes to source files if problems are found"),
            );
        let matches = app.clone().get_matches();
        let mut options = self;
        options.filepath = match matches.value_of("FILEPATH") {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        if matches.is_present("DEBUG") {
            options.debug = true;
        }
        if matches.is_present("READONLY") {
            options.fixup_file = false;
        }
        if options.filepath == None {
            app.write_help(&mut std::io::stdout())
                .expect("failed to write to stdout");
            println!();
        }
        return options;
    }
}
