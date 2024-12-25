use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    input
        .iter()
        .map(|num| {
            let mut result = *num;
            for _ in 0..2000 {
                result = rng(result);
            }
            result
        })
        .sum()
}

fn rng(num: usize) -> usize {
    let mut result;
    let mut tmp = num * 64;
    result = num ^ tmp;
    result %= 16777216;

    tmp = result / 32;
    result ^= tmp;
    result %= 16777216;

    tmp = result * 2048;
    result ^= tmp;
    result %= 16777216;

    result
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut sequences = Vec::new();
    input.iter().for_each(|num| {
        let mut sequence = Vec::new();
        let mut result = *num;
        let mut prev = (result % 10) as i8;
        for _ in 0..2000 {
            result = rng(result);
            sequence.push(((result % 10) as i8, (result % 10) as i8 - prev));
            prev = (result % 10) as i8;
        }
        sequences.push(sequence);
    });

    (0..130_321)
        .map(|pattern| {
            let pattern = get_pattern_from_index(pattern);

            sequences
                .iter()
                .map(|sequence| {
                    let pos = sequence
                        .windows(4)
                        .find_position(|win| (win[0].1, win[1].1, win[2].1, win[3].1) == pattern);
                    match pos {
                        Some(pos) => sequence[pos.0 + 3].0 as usize,
                        None => 0,
                    }
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn get_pattern_from_index(index: usize) -> (i8, i8, i8, i8) {
    let mut tmp = index;
    let one = (tmp % 19) as i8 - 9;
    tmp /= 19;
    let two = (tmp % 19) as i8 - 9;
    tmp /= 19;
    let three = (tmp % 19) as i8 - 9;
    tmp /= 19;
    let four = (tmp % 19) as i8 - 9;

    (one, two, three, four)
}

#[cfg(test)]
mod tests {
    use crate::day22::{input_generator, solve_part1 /*solve_part2*/};

    #[test]
    fn task_sample() {
        let input1 = input_generator(
            r#"1
10
100
2024"#,
        );
        assert_eq!(solve_part1(&input1), 37327623);
        /*let input2 = input_generator(r#"1
        2
        3
        2024"#);*/
        //assert_eq!(solve_part2(&input2), 23);
    }
}
