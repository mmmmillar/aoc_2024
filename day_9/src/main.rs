use common::load_lines;

fn diff_sequence(parent: &Vec<i32>) -> Vec<i32> {
    parent.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn all_zero(sequence: &Vec<i32>) -> bool {
    sequence.iter().all(|&x| x == 0)
}

fn part_1() -> i32 {
    load_lines("input.txt")
        .map(|line| {
            let mut sequences = Vec::new();

            let next_sequence = line
                .split(" ")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            sequences.push(next_sequence);

            loop {
                let last = sequences.get(sequences.len() - 1).unwrap();

                if all_zero(last) {
                    break;
                }

                sequences.push(diff_sequence(last));
            }

            sequences.iter().map(|v| v.last().unwrap()).sum::<i32>()
        })
        .sum()
}

fn part_2() -> i32 {
    0
}

fn main() {
    println!("PART 1: {}", part_1());
    println!("PART 2: {}", part_2());
}
