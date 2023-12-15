use std::collections::HashMap;

use common::load_lines;
use regex::Regex;

fn parse_input(n_repeats: usize) -> Vec<(Vec<char>, Vec<usize>)> {
    let spring_regex = Regex::new(r"(?<record>[.#?]+) (?<counts>[\d,]+)").unwrap();

    load_lines("input.txt")
        .map(|line| {
            spring_regex
                .captures(&line)
                .map(|c| {
                    let mut record =
                        vec![String::from(c.name("record").unwrap().as_str()); n_repeats]
                            .join("?")
                            .chars()
                            .collect::<Vec<char>>();
                    record.push('.'); // turns out adding this padding at the end makes the match logic much simpler

                    let counts = vec![c.name("counts").unwrap().as_str(); n_repeats]
                        .join(",")
                        .split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();

                    (record, counts)
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn num_matches(record: &[char], counts: &[usize], memo: &mut HashMap<String, u64>) -> u64 {
    // #'s unaccounted for
    if counts.is_empty() {
        match record.contains(&'#') {
            true => return 0,
            false => return 1,
        }
    }

    // more in counts than in record (counts.len() required as separator count)
    if record.len() < counts.iter().sum::<usize>() + counts.len() {
        return 0;
    }

    let key = format!("{}_{}", record.iter().collect::<String>(), counts.len());

    if let Some(&prev) = memo.get(&key) {
        return prev;
    }

    let mut matches = 0;

    if record[0] != '#' {
        // assume operational
        matches += num_matches(&record[1..], counts, memo);
    }

    let next_count = counts[0];

    if !record[..next_count].contains(&'.') && record[next_count] != '#' {
        // assume damaged
        matches += num_matches(&record[next_count + 1..], &counts[1..], memo);
    }

    memo.insert(key, matches);

    matches
}

fn part_1() -> u64 {
    parse_input(1)
        .iter()
        .map(|(record, counts)| {
            let mut memo = HashMap::new();
            num_matches(record, counts, &mut memo)
        })
        .sum()
}

fn part_2() -> u64 {
    parse_input(5)
        .iter()
        .map(|(record, counts)| {
            let mut memo = HashMap::new();
            num_matches(record, counts, &mut memo)
        })
        .sum()
}

fn main() {
    println!("PART 1: {}", part_1());
    println!("PART 2: {}", part_2());
}
