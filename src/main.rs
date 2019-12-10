use rand::prelude::*;
use std::io::{Write, Error, stdout};
use std::env::args;

mod noun;
mod adjective;

use noun::NOUNS;
use adjective::ADJECTIVES;

fn main() -> Result<(), Error> {
    let count = args()
        .nth(1).unwrap_or_else(|| String::from("1"))
        .parse::<usize>().unwrap_or(1);

    let std_out = &mut stdout();
    let rng = &mut thread_rng();

    for _i in 0..count {
        writeln!(std_out, "{}", three_words(rng))?
    }

    Ok(())
}

fn three_words(rng: &mut ThreadRng) -> String {
    let adjs = ADJECTIVES.choose_multiple(rng, 2).collect::<Vec<_>>();
    let noun = NOUNS.choose(rng).expect("Slice is empty!");

    format!("{} {} {}", adjs[0], adjs[1], noun)
}
