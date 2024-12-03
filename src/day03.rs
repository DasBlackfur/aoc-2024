use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    let regex = Regex::new(r"(?:mul\((\d\d?\d?),(\d\d?\d?)\)|(do)\(()\)|(don't)\(()\))").unwrap();
    regex.captures_iter(input).map(|mul| {
        mul.extract::<2>().1.iter().filter(|string| !string.is_empty()).map(|string| string.to_string()).collect()
    }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<String>>) -> u32 {
    input.iter().map(|num_pair| {
        if num_pair.len() == 2 {
            num_pair[0].parse::<u32>().unwrap() * num_pair[1].parse::<u32>().unwrap()
        } else {
            0
        }
    }).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<String>>) -> u32 {
    let mut do_mul = true;
    let do_mul_ref = &mut do_mul;
    input.iter().map(|num_pair| {
        if num_pair[0] == "do" {
            *do_mul_ref = true;
            0
        } else if num_pair[0] == "don't" {
            *do_mul_ref = false;
            0
        } else {
            if *do_mul_ref {
                num_pair[0].parse::<u32>().unwrap() * num_pair[1].parse::<u32>().unwrap()
            } else {
                0
            }
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day03::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(solve_part1(&input), 161);
        assert_eq!(solve_part2(&input), 48);
    }
}
