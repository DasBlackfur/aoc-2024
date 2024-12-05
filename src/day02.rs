use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .map(|report| {
            report
                .replace("\r", "")
                .split(" ")
                .map(|number| number.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|report| {
            let mut last_num = report[0];
            let valid_lesser = report[1..]
                .iter()
                .filter(|num| {
                    let diff = last_num.abs_diff(**num);
                    if !(last_num > **num && (1..=3).contains(&diff)) {
                        last_num = **num;
                        return true;
                    }
                    last_num = **num;
                    false
                })
                .count();
            let mut last_num = report[0];
            let valid_greater = report[1..]
                .iter()
                .filter(|num| {
                    let diff = last_num.abs_diff(**num);
                    if !(last_num < **num && (1..=3).contains(&diff)) {
                        last_num = **num;
                        return true;
                    }
                    last_num = **num;
                    false
                })
                .count();
            valid_greater.min(valid_lesser) == 0
        })
        .count() as u32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .filter(|report| {
            (0..report.len())
                .map(|index| {
                    let mut cut_report = (*report).clone();
                    cut_report.remove(index);
                    let mut last_num = cut_report[0];
                    let valid_lesser = cut_report[1..]
                        .iter()
                        .filter(|num| {
                            let diff = last_num.abs_diff(**num);
                            if !(last_num > **num && (1..=3).contains(&diff)) {
                                last_num = **num;
                                return true;
                            }
                            last_num = **num;
                            false
                        })
                        .count();
                    let mut last_num = cut_report[0];
                    let valid_greater = cut_report[1..]
                        .iter()
                        .filter(|num| {
                            let diff = last_num.abs_diff(**num);
                            if !(last_num < **num && (1..=3).contains(&diff)) {
                                last_num = **num;
                                return true;
                            }
                            last_num = **num;
                            false
                        })
                        .count();
                    valid_greater.min(valid_lesser) == 0
                })
                .max()
                .unwrap()
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day02::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator("21 22 25 23 24");
        assert_eq!(solve_part1(&input), 0);
        assert_eq!(solve_part2(&input), 1);
    }
}
