
#[macro_use]
mod log;
mod options;

fn main() {
    let options = options::Options::new_default().with_cmd_line();
    if let Some(s) = options.filepath {
        bft_log!(options, "Loading source code: {}", s);
        /*
        if options.debug {
            eprintln!("file {}:", &file);
            eprint!("opening...");
        }
        let mut f = match File::open(file.clone()) {
            Result::Ok(v) => v,
            Result::Err(e) => {
                eprintln!("file {} not found: {}", &file, e);
                continue;
            }
        };

        if debug {
            eprint!("reading...");
        }
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Result::Ok(_) => (),
            Result::Err(e) => {
                eprintln!("error reading {}: {}", &file, e);
                continue;
            }
        }

        if debug {
            eprint!("tokenizing...");
        }
        let mut values: Vec<String> = vec![];
        for i in re.find_iter(&contents) {
            values.push(contents[i.start()..i.end()].to_string());
        }*/
    }
}
