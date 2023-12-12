use common::load_lines;

fn diff_sequence(parent: &Vec<i32>) -> Vec<i32> {
    parent.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn create_sequences(line: String) -> Vec<Vec<i32>> {
    let mut sequences = vec![line
        .split(" ")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>()];

    loop {
        let last = sequences.last().unwrap();

        if last.iter().sum::<i32>() == 0 {
            break;
        }

        sequences.push(diff_sequence(last));
    }

    sequences
}

fn part_1() -> i32 {
    load_lines("input.txt")
        .map(|line| {
            create_sequences(line)
                .iter()
                .map(|v| v.last().unwrap())
                .sum::<i32>()
        })
        .sum()
}

fn part_2() -> i32 {
    load_lines("input.txt")
        .map(|line| {
            create_sequences(line)
                .iter()
                .rev()
                .skip(1)
                .fold(0, |acc, next| next.first().unwrap() - acc)
        })
        .sum()
}

fn main() {
    println!("PART 1: {}", part_1());
    println!("PART 2: {}", part_2());
}
