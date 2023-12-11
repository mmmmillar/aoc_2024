use common::load_lines;
use regex::Regex;

#[derive(Debug)]
struct Round {
    green: u32,
    red: u32,
    blue: u32,
}

fn colour_count(round_str: &str, colour: &str) -> u32 {
    let captures = Regex::new(&format!(r"(?<{colour}>\d+) {}", colour))
        .unwrap()
        .captures(round_str);

    match captures {
        Some(c) => c.name(colour).unwrap().as_str().parse::<u32>().unwrap(),
        None => 0,
    }
}

fn part_1() -> u32 {
    let lines = load_lines("input.txt");
    let game_regex = Regex::new(r"^Game (\d+): +(.*)$").unwrap();

    lines
        .map(|line: String| {
            let (_, [id, body]) = game_regex.captures(&line).unwrap().extract();

            let is_valid = body
                .split(';')
                .map(|r| Round {
                    green: colour_count(r, "green"),
                    blue: colour_count(r, "blue"),
                    red: colour_count(r, "red"),
                })
                .filter(|round| round.red > 12 || round.green > 13 || round.blue > 14)
                .collect::<Vec<Round>>()
                .len()
                == 0;

            match is_valid {
                true => id.parse::<u32>().unwrap(),
                false => 0,
            }
        })
        .sum()
}

fn part_2() -> u32 {
    let lines = load_lines("input.txt");
    let game_regex = Regex::new(r"^Game (\d+): +(.*)$").unwrap();

    lines
        .map(|line: String| {
            let (_, [_, body]) = game_regex.captures(&line).unwrap().extract();

            let round = body
                .split(';')
                .map(|r| Round {
                    green: colour_count(r, "green"),
                    blue: colour_count(r, "blue"),
                    red: colour_count(r, "red"),
                })
                .fold(
                    Round {
                        green: 0,
                        blue: 0,
                        red: 0,
                    },
                    |mut acc, r| {
                        if r.green > acc.green {
                            acc.green = r.green
                        }

                        if r.blue > acc.blue {
                            acc.blue = r.blue
                        }

                        if r.red > acc.red {
                            acc.red = r.red
                        }

                        acc
                    },
                );

            round.green * round.blue * round.red
        })
        .sum()
}

fn main() {
    println!("PART 1: {}", part_1());
    println!("PART 1: {}", part_2());
}
