use rand::prelude::*;
use std::io::{Write, Error, stdout};
use std::fmt;
use clap::ArgMatches;

mod cli;
mod noun;
mod adjective;

use noun::NOUNS;
use adjective::ADJECTIVES;

fn main() -> Result<(), Error> {
    let matches = cli::app().get_matches();
    let count = matches.value_of("count").unwrap().parse().unwrap();

    let mut std_out = stdout();
    let mut rng = thread_rng();

    for _i in 0..count {
        writeln!(&mut std_out, "{}", Words::from((&mut rng, &matches)))?;
    }

    Ok(())
}

use Words::*;
enum Words {
    ThreeWords {
        adj0: String,
        adj1: String,
        noun: String,
        delimiter: String,
    },
    FourWords {
        adj0: String,
        noun0: String,
        adj1: String,
        noun1: String,
        delimiter: String,
    },
}

impl Words {
    fn new3(adjs: Rando, nouns: Rando, del: &str) -> Self {
        ThreeWords {
            adj0: adjs[0].to_string(),
            adj1: adjs[1].to_string(),
            noun: nouns[0].to_string(),
            delimiter: del.to_string(),
        }
    }

    fn new4(adjs: Rando, nouns: Rando, del: &str) -> Self {
        FourWords {
            adj0: adjs[0].to_string(),
            noun0: nouns[0].to_string(),
            adj1: adjs[1].to_string(),
            noun1: nouns[1].to_string(),
            delimiter: del.to_string(),
        }
    }

}

impl fmt::Display for Words {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreeWords { adj0, adj1, noun, delimiter }
                => write!(f, "{}{del}{}{del}{}", adj0, adj1, noun, del=delimiter),
            FourWords { adj0, noun0, adj1, noun1, delimiter }
                => write!(f, "{}{del}{}{del}{}{del}{}", adj0, noun0, adj1, noun1, del=delimiter),
        }
    }
}

type RangeMatches<'rng, 'matches> = (&'rng mut ThreadRng, &'matches ArgMatches<'static>);

impl<'rng, 'matches> From<RangeMatches<'rng, 'matches>> for Words {
    fn from(rng_matches: RangeMatches) -> Self {
        let (rng, matches) = rng_matches;

        let delimiter = matches.value_of("delimiter").unwrap();

        let (adjs, nouns) = if matches.is_present("length") || matches.is_present("limit") {
            adjs_nouns_ltd(rng, &matches)
        } else {
            adjs_nouns(rng)
        };

        if matches.is_present("four") {
            Words::new4(adjs, nouns, delimiter)
        } else {
            Words::new3(adjs, nouns, delimiter)
        }
    }
}

type Rando<'a> = Vec<&'a &'static str>;

fn adjs_nouns(rng: &mut ThreadRng) -> (Rando, Rando) {
    (
        ADJECTIVES.choose_multiple(rng, 2).collect(),
        NOUNS.choose_multiple(rng, 2).collect(),
    )
}

/// Filter out words longer than a certain length
fn adjs_nouns_ltd<'a>(rng: &mut ThreadRng, matches: &ArgMatches) -> (Rando<'a>, Rando<'a>) {
    let word_len = |word: &&&str| word.len() <= matches.value_of("length").unwrap_or("12").parse().unwrap();
    let adjs = ADJECTIVES
        .iter()
        .filter(word_len)
        .choose_multiple(rng, 2);
    let nouns = NOUNS
        .iter()
        .filter(word_len)
        .choose_multiple(rng, 2);

    if let Some(limit) = matches.value_of("limit") {
        let del_len = matches.value_of("delimiter").unwrap_or(" ").len();
        let gaps = if matches.is_present("four") {
            3
        } else {
            2
        };
        let adds = del_len * gaps;

        // try it again if it's too long
        if char_count(&adjs, &nouns, adds, matches.is_present("four")) > limit.parse().unwrap() {
            adjs_nouns_ltd(rng, matches)
        } else {
            (adjs, nouns)
        }
    } else {
        (adjs, nouns)
    }
}

fn char_count(adjs: &Rando, nouns: &Rando, adds: usize, four: bool) -> usize {
    let adjs = adjs.iter().flat_map(|s| s.chars()).count();
    let nouns = if four {
        nouns.iter().flat_map(|s| s.chars()).count()
    } else {
        nouns[0].len()
    };

    adjs + nouns + adds
}

#[test]
fn possible_combos() {
    eprintln!("Possible combos (3): {}", ADJECTIVES.len() * ADJECTIVES.len() * NOUNS.len());
    eprintln!("Possible combos (4): {}", ADJECTIVES.len() * NOUNS.len() * ADJECTIVES.len() * NOUNS.len());
}
