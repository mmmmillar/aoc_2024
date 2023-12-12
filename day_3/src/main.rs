use common::load_lines;
use regex::Regex;

#[derive(Debug)]
struct Num {
    x: i32,
    y: i32,
    len: i32,
    value: String,
}

impl Num {
    fn is_part(&self, symbols: &Vec<(i32, i32)>) -> bool {
        ((self.x - 1)..(self.x + self.len + 1))
            .flat_map(|x| ((self.y - 1)..(self.y + 2)).map(move |y| (x, y)))
            .filter(|n| n.0 > -1 && n.1 > -1)
            .filter(|n| symbols.contains(&(n.0, n.1)))
            .collect::<Vec<(i32, i32)>>()
            .len()
            > 0
    }
}

fn part_1() -> u32 {
    let num_regex = Regex::new(r"(?<number>\d+)").unwrap();
    let symbol_regex = Regex::new(r"(?<symbol>[^\s\d.])").unwrap();

    let symbols = load_lines("input.txt")
        .enumerate()
        .fold(vec![], |mut acc, (i, line)| {
            let symbols = symbol_regex
                .find_iter(&line)
                .map(|m| (m.start() as i32, i as i32));
            acc.extend(symbols);
            acc
        });

    load_lines("input.txt")
        .enumerate()
        .fold(vec![], |mut acc, (i, line)| {
            let numbers = num_regex.find_iter(&line).map(|m| Num {
                x: m.start() as i32,
                y: i as i32,
                len: (m.end() - m.start()) as i32,
                value: String::from(m.as_str()),
            });
            acc.extend(numbers);
            acc
        })
        .iter()
        .filter(|&n| n.is_part(&symbols))
        .map(|n| n.value.parse::<u32>().unwrap())
        .sum()
}

fn main() {
    println!("PART 1: {}", part_1());
}
