use common::load_lines;

fn replace_words(line: String, nums: [&str; 9]) -> String {
    let m = nums
        .iter()
        .enumerate()
        .filter_map(|(num_i, num_str)| line.find(num_str).map(|pos| (num_str, num_i, pos)))
        .min_by_key(|&(_, _, pos)| pos)
        .map(|(num_str, num_i, pos)| (num_str, format!("{}", num_i + 1), pos));

    if let Some((num_str, num, pos)) = m {
        return line.replacen(num_str, &num, 1)[..pos + 1].to_string();
    }

    line
}

fn part_1() -> i32 {
    let lines = load_lines("input.txt");

    lines
        .map(|text| {
            let digits = text
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<char>>();

            let num = match digits.len() {
                0 => 0,
                x => {
                    let mut num_str = String::new();
                    num_str.push(digits[0]);
                    num_str.push(digits[x - 1]);
                    num_str.parse::<i32>().unwrap()
                }
            };

            num
        })
        .fold(0, |acc, num| acc + num)
}

fn part_2() -> i32 {
    let lines = load_lines("input.txt");

    lines
        .map(|line| {
            // println!("ROW INPUT: {}", line);

            let front = replace_words(
                line.clone(),
                [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ],
            );

            let back = replace_words(
                line.chars().rev().collect::<String>(),
                [
                    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
                ],
            )
            .chars()
            .rev()
            .collect::<String>();

            let full = format!("{}{}", front, back);

            // println!("UPDATED ROW INPUT: {}", full);

            let digits = full
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>();

            let num = match digits.len() {
                0 => 0,
                x => {
                    let mut num_str = String::new();
                    num_str.push(digits[0]);
                    num_str.push(digits[x - 1]);
                    num_str.parse::<i32>().unwrap()
                }
            };

            // println!("ROW CALIBRATION VALUE: {}\n", num);

            num
        })
        .sum()
}

fn main() {
    println!("PART 1: {}", part_1());
    println!("PART 2: {}", part_2());
}
