use rand::prelude::*;
use std::io::{Write, Error, stdout};

mod cli;
mod noun;
mod adjective;

use noun::NOUNS;
use adjective::ADJECTIVES;

fn main() -> Result<(), Error> {
    let matches = cli::app().get_matches();
    let count = matches.value_of("count").unwrap().parse().unwrap();

    let std_out = &mut stdout();
    let rng = &mut thread_rng();

    for _i in 0..count {
        writeln!(std_out, "{}", words(rng, &matches))?
    }

    Ok(())
}

fn words(rng: &mut ThreadRng, matches: &clap::ArgMatches) -> String {
    let delimiter = matches.value_of("delimiter").unwrap();
    let word_len = |word: &&&str| word.len() <= matches.value_of("length").unwrap().parse().unwrap();

    let adjs = ADJECTIVES
        .iter()
        .filter(word_len)
        .choose_multiple(rng, 2);
    let nouns = NOUNS
        .iter()
        .filter(word_len)
        .choose_multiple(rng, 2);

    if matches.is_present("four") {
        format!("{}{del}{}{del}{}{del}{}", adjs[0], nouns[0], adjs[1], nouns[1], del=delimiter)
    } else {
        format!("{}{del}{}{del}{}", adjs[0], adjs[1], nouns[0], del=delimiter)
    }   
}

#[test]
fn possible_combos() {
    eprintln!("Possible combos (3): {}", ADJECTIVES.len() * ADJECTIVES.len() * NOUNS.len());
    eprintln!("Possible combos (4): {}", ADJECTIVES.len() * NOUNS.len() * ADJECTIVES.len() * NOUNS.len());
}
