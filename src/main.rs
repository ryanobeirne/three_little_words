use rand::prelude::*;
use std::io::{Write, Error, stdout};
use std::env::args;

mod noun;
mod adjective;

fn main() -> Result<(), Error> {
    let count = args()
        .nth(1).unwrap_or(String::from("1"))
        .parse::<usize>().unwrap_or(1);

    let rng = &mut thread_rng();

    for _i in 0..count {
        writeln!(stdout(), "{}", three_words(rng))?
    }

    Ok(())
}

fn three_words(rng: &mut ThreadRng) -> String {
    let adjs: Vec<&str> = adjective::ADJECTIVES.choose_multiple(rng, 2)
        .map(|s| (*s).into())
        .collect();

    let noun = noun::NOUNS.choose(rng).unwrap();

    format!("{} {} {}", adjs[0], adjs[1], noun)
}