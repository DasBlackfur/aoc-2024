use std::{collections::HashMap, iter::successors};

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(" ").map(|num| num.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut lookup: HashMap<(usize, u8), usize> = HashMap::new();
    input
        .iter()
        .map(move |num| solve(&mut lookup, *num, 25))
        .sum()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut lookup: HashMap<(usize, u8), usize> = HashMap::new();
    input.iter().map(|num| solve(&mut lookup, *num, 75)).sum()
}

fn solve(lookup: &mut HashMap<(usize, u8), usize>, num: usize, remaining_iters: u8) -> usize {
    if remaining_iters == 0 {
        return 1;
    }
    if let Some(cached) = lookup.get(&(num, remaining_iters)) {
        return *cached;
    }
    let result = {
        if num == 0 {
            solve(lookup, 1, remaining_iters - 1)
        } else if digits(num) % 2 == 0 {
            let num_str = num.to_string();
            solve(
                lookup,
                num_str[..(num_str.len() / 2)].parse().unwrap(),
                remaining_iters - 1,
            ) + solve(
                lookup,
                num_str[(num_str.len() / 2)..].parse().unwrap(),
                remaining_iters - 1,
            )
        } else {
            solve(lookup, num * 2024, remaining_iters - 1)
        }
    };

    lookup.insert((num, remaining_iters), result);
    result
}

fn digits(n: usize) -> usize {
    successors(Some(n), |&i| (i >= 10).then_some(i / 10)).count()
}

#[cfg(test)]
mod tests {
    use crate::day11::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator("125 17");
        assert_eq!(solve_part1(&input), 55312);
        assert_eq!(solve_part2(&input), 65601038650482);
    }
}
