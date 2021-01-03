use clap::{App, Arg};

pub fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_BIN_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("count")
                .help("The number of random passwords to generate")
                .short("n")
                .long("count")
                .validator(is_int)
                .default_value("20"),
        )
        .arg(
            Arg::with_name("four")
                .help("Generate four words in the format [adjective noun adjective noun]")
                .short("4")
                .long("four")
        )
        .arg(
            Arg::with_name("delimiter")
                .help("Separate words with a delimeter")
                .short("d")
                .long("delimiter")
                .default_value(" ")
        )
        .arg(
            Arg::with_name("length")
                .help("Limit each word length to N characters")
                .short("l")
                .value_name("N")
                .long("length")
                .validator(is_int_gt3)
        )
        .arg(
            Arg::with_name("limit")
                .help("Limit total length to N characters")
                .short("L")
                .value_name("N")
                .long("limit")
                .conflicts_with("length")
                .conflicts_with("four")
                .validator(is_int_gt20)
        )
}

fn is_int(s: String) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("invalid integer: `{}`", s)),
    }
}

fn is_int_gt3(s: String) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(int) => {
            if int > 3 {
                Ok(())
            } else {
                Err(format!("integer must be greater than 3"))
            }
        }
        Err(_) => Err(format!("invalid integer: `{}`", s)),
    }
}

fn is_int_gt20(s: String) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(int) => {
            if int > 20 {
                Ok(())
            } else {
                Err(format!("integer must be greater than 20"))
            }
        }
        Err(_) => Err(format!("invalid integer: `{}`", s)),
    }
}
