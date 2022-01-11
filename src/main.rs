use clap::ArgMatches;
use rand::prelude::*;
use std::fmt;
use std::io::{stdout, Error, Write};

mod adjective;
mod cli;
mod noun;

use adjective::ADJECTIVES;
use noun::NOUNS;

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
    OneWord {
        word: String,
    },
    TwoWords {
        adj: String,
        noun: String,
        delimiter: String,
    },
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
    fn new1(adjs: Rando, nouns: Rando) -> Self {
        let mut rng = thread_rng();

        if rng.gen() {
            OneWord {
                word: nouns[rng.gen_range(0..nouns.len())].to_string(),
            }
        } else {
            OneWord {
                word: adjs[rng.gen_range(0..adjs.len())].to_string(),
            }
        }
    }

    fn new2(adjs: Rando, nouns: Rando, del: &str) -> Self {
        TwoWords {
            adj: adjs[0].to_string(),
            noun: nouns[0].to_string(),
            delimiter: del.to_string(),
        }
    }

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
            OneWord { word: noun }
                => write!(f, "{}", noun),
            TwoWords { adj, noun, delimiter }
                => write!(f, "{}{del}{}", adj, noun, del=delimiter),
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

        match WordCount::from(matches) {
            One => Words::new1(adjs, nouns),
            Two => Words::new2(adjs, nouns, delimiter),
            Three => Words::new3(adjs, nouns, delimiter),
            Four => Words::new4(adjs, nouns, delimiter),
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
fn adjs_nouns_ltd<'a>(
    rng: &mut ThreadRng,
    matches: &ArgMatches<'static>,
) -> (Rando<'a>, Rando<'a>) {
    let word_len =
        |word: &&&str| word.len() <= matches.value_of("length").unwrap_or("12").parse().unwrap();
    let adjs = ADJECTIVES.iter().filter(word_len).choose_multiple(rng, 2);
    let nouns = NOUNS.iter().filter(word_len).choose_multiple(rng, 2);

    if let Some(limit) = matches.value_of("limit") {
        if char_count(&adjs, &nouns, &matches) > limit.parse().unwrap() {
            // try it again if it's too long
            adjs_nouns_ltd(rng, matches)
        } else {
            (adjs, nouns)
        }
    } else {
        (adjs, nouns)
    }
}

fn char_count(adjs: &Rando, nouns: &Rando, matches: &ArgMatches<'static>) -> usize {
    let word_count = WordCount::from(matches);
    let del_len = matches.value_of("delimiter").unwrap().len();
    let adds = (word_count as usize - 1) * del_len;

    // Find the length of all the words we'll actually be using
    let (adjs, nouns) = match word_count {
        One => (0, nouns[0].len()),
        Two => (adjs[0].len(), nouns[0].len()),
        Three => (adjs.iter().flat_map(|s| s.chars()).count(), nouns[0].len()),
        Four => (
            adjs.iter().flat_map(|s| s.chars()).count(),
            nouns.iter().flat_map(|s| s.chars()).count(),
        ),
    };

    adjs + nouns + adds
}

use WordCount::*;
#[derive(Clone, Copy)]
enum WordCount {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}

impl Default for WordCount {
    fn default() -> Self {
        Three
    }
}

impl std::str::FromStr for WordCount {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1" => One,
            "2" => Two,
            "3" => Three,
            "4" => Four,
            x => return Err(format!("invalid WordCount: `{}`", x)),
        })
    }
}

impl<'a> From<&'a ArgMatches<'static>> for WordCount {
    fn from(matches: &'a ArgMatches) -> Self {
        matches
            .value_of("word-count")
            .unwrap_or("3")
            .parse()
            .unwrap()
    }
}

#[test]
fn possible_combos() {
    eprintln!("Possible combos (1): {}", NOUNS.len() + ADJECTIVES.len());
    eprintln!("Possible combos (2): {}", ADJECTIVES.len() * NOUNS.len());
    eprintln!(
        "Possible combos (3): {}",
        ADJECTIVES.len() * ADJECTIVES.len() * NOUNS.len()
    );
    eprintln!(
        "Possible combos (4): {}",
        ADJECTIVES.len() * NOUNS.len() * ADJECTIVES.len() * NOUNS.len()
    );
}
