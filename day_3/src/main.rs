use common::load_lines;
use regex::Regex;

#[derive(Debug)]
struct Num {
    x: i32,
    y: i32,
    len: i32,
    value: u32,
}

impl Num {
    // also includes the actual nums coordinates but this does not matter
    fn adjacent_coordinates(&self) -> Vec<(i32, i32)> {
        ((self.x - 1)..(self.x + self.len + 1))
            .flat_map(|x| ((self.y - 1)..(self.y + 2)).map(move |y| (x, y)))
            .collect()
    }

    fn is_part(&self, symbols: &Vec<(i32, i32)>) -> bool {
        self.adjacent_coordinates()
            .iter()
            .filter(|n| n.0 > -1 && n.1 > -1)
            .filter(|n| symbols.contains(&(n.0, n.1)))
            .collect::<Vec<&(i32, i32)>>()
            .len()
            > 0
    }
}

#[derive(Debug)]
struct Gear {
    x: i32,
    y: i32,
}

impl Gear {
    fn ratio(&self, numbers: &Vec<Num>) -> u32 {
        let adjacent_numbers = numbers
            .iter()
            .filter(|n| n.adjacent_coordinates().contains(&(self.x, self.y)))
            .collect::<Vec<_>>();

        match adjacent_numbers.len() {
            2 => adjacent_numbers.iter().map(|n| n.value).product(),
            _ => 0,
        }
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
                value: m.as_str().parse::<u32>().unwrap(),
            });
            acc.extend(numbers);
            acc
        })
        .iter()
        .filter(|&n| n.is_part(&symbols))
        .map(|n| n.value)
        .sum()
}

fn part_2() -> u32 {
    let num_regex = Regex::new(r"(?<number>\d+)").unwrap();
    let gear_regex = Regex::new(r"(?<gear>[\*])").unwrap();

    let numbers = load_lines("input.txt")
        .enumerate()
        .fold(vec![], |mut acc, (i, line)| {
            let numbers = num_regex.find_iter(&line).map(|m| Num {
                x: m.start() as i32,
                y: i as i32,
                len: (m.end() - m.start()) as i32,
                value: m.as_str().parse::<u32>().unwrap(),
            });
            acc.extend(numbers);
            acc
        });

    load_lines("input.txt")
        .enumerate()
        .fold(vec![], |mut acc, (i, line)| {
            let gears = gear_regex.find_iter(&line).map(|m| Gear {
                x: m.start() as i32,
                y: i as i32,
            });
            acc.extend(gears);
            acc
        })
        .iter()
        .map(|g| g.ratio(&numbers))
        .sum()
}

fn main() {
    println!("PART 1: {}", part_1());
    println!("PART 2: {}", part_2());
}
