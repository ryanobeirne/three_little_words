use rand::prelude::*;
use std::io::{Write, Error, stdout};
use clap::{App, Arg};

mod noun;
mod adjective;

use noun::NOUNS;
use adjective::ADJECTIVES;

fn main() -> Result<(), Error> {
    let matches = app().get_matches();
    let count = matches.value_of("count").unwrap().parse().unwrap();
    let four = matches.is_present("four");
    let delimiter = matches.value_of("delimiter").unwrap();

    let std_out = &mut stdout();
    let rng = &mut thread_rng();

    for _i in 0..count {
        writeln!(std_out, "{}",
            if four {
                four_words(rng, delimiter)
            } else {
                three_words(rng, delimiter)
            })?
    }

    Ok(())
}

fn three_words(rng: &mut ThreadRng, delimiter: &str) -> String {
    let adjs = ADJECTIVES.choose_multiple(rng, 2).collect::<Vec<_>>();
    let noun = NOUNS.choose(rng).expect("Slice is empty!");

    format!("{}{del}{}{del}{}", adjs[0], adjs[1], noun, del=delimiter)
}

fn four_words(rng: &mut ThreadRng, delimiter: &str) -> String {
    let adjs = ADJECTIVES.choose_multiple(rng, 2).collect::<Vec<_>>();
    let nouns = NOUNS.choose_multiple(rng, 2).collect::<Vec<_>>();

    format!("{}{del}{}{del}{}{del}{}", adjs[0], nouns[0], adjs[1], nouns[1], del=delimiter)
}

fn app() -> App<'static, 'static> {
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
}

fn is_int(s: String) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("invalid integer: `{}`", s)),
    }
}

#[test]
fn possible_combos() {
    eprintln!("Possible combos (3): {}", ADJECTIVES.len() * ADJECTIVES.len() * NOUNS.len());
    eprintln!("Possible combos (4): {}", ADJECTIVES.len() * NOUNS.len() * ADJECTIVES.len() * NOUNS.len());
}
