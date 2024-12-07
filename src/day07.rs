use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .split("\n")
        .map(|line| {
            let mut line_parts = line.split(": ");
            (
                line_parts.next().unwrap().parse().unwrap(),
                line_parts
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|calibration| {
            let mut op_vec = vec![0; calibration.1.len() - 1];
            let mut run_count = 0;

            while run_count <= 2_u64.pow(op_vec.len() as u32) {
                let mut result = 0;
                for (pos, op) in op_vec.iter().enumerate() {
                    if pos == 0 {
                        match op {
                            0 => result = calibration.1[0] + calibration.1[1],
                            1 => result = calibration.1[0] * calibration.1[1],
                            _ => unreachable!(),
                        }
                    } else {
                        match op {
                            0 => result += calibration.1[pos + 1],
                            1 => result *= calibration.1[pos + 1],
                            _ => unreachable!(),
                        }
                    }
                }
                if result == calibration.0 {
                    return true;
                }
                run_count += 1;
                let mut has_wrapped = 0;
                let has_wrapped_ref = &mut has_wrapped;
                op_vec = op_vec
                    .iter_mut()
                    .map(|op| match (op, *has_wrapped_ref) {
                        (0, 0) => {
                            *has_wrapped_ref = 1;
                            1
                        }
                        (1, 0) => {
                            *has_wrapped_ref = 2;
                            0
                        }
                        (op, 1) => *op,
                        (0, 2) => {
                            *has_wrapped_ref = 1;
                            1
                        }
                        (1, 2) => 0,
                        (_, _) => unreachable!(),
                    })
                    .collect();
            }
            false
        })
        .map(|valid| valid.0)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|calibration| {
            let mut op_vec = vec![0; calibration.1.len() - 1];
            let mut run_count = 0;

            while run_count < 3_u64.pow(op_vec.len() as u32) {
                let mut result = 0;
                for (pos, op) in op_vec.iter().enumerate() {
                    if pos == 0 {
                        match op {
                            0 => result = calibration.1[0] + calibration.1[1],
                            1 => result = calibration.1[0] * calibration.1[1],
                            2 => {
                                result = (calibration.1[0].to_string()
                                    + &calibration.1[1].to_string())
                                    .parse()
                                    .unwrap()
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match op {
                            0 => result += calibration.1[pos + 1],
                            1 => result *= calibration.1[pos + 1],
                            2 => {
                                result = (result.to_string() + &calibration.1[pos + 1].to_string())
                                    .parse()
                                    .unwrap()
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                if result == calibration.0 {
                    return true;
                }
                run_count += 1;
                let mut has_wrapped = 0;
                let has_wrapped_ref = &mut has_wrapped;
                op_vec = op_vec
                    .iter_mut()
                    .map(|op| match (op, *has_wrapped_ref) {
                        (0, 0) => {
                            *has_wrapped_ref = 1;
                            1
                        }
                        (1, 0) => {
                            *has_wrapped_ref = 1;
                            2
                        }
                        (2, 0) => {
                            *has_wrapped_ref = 2;
                            0
                        }
                        (op, 1) => *op,
                        (0, 2) => {
                            *has_wrapped_ref = 1;
                            1
                        }
                        (1, 2) => {
                            *has_wrapped_ref = 1;
                            2
                        }
                        (2, 2) => 0,
                        (_, _) => unreachable!(),
                    })
                    .collect();
            }
            false
        })
        .map(|valid| valid.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day07::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
        );
        assert_eq!(solve_part1(&input), 3749);
        assert_eq!(solve_part2(&input), 11387);
    }
}
