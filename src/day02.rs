use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input.split("\n").map(|report| {
        report.split(" ").map(|number| number.parse::<u32>().unwrap()).collect()
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input.iter().filter(|report| {
        let mut last_num = report[0];
        let valid_lesser = report[1..].iter().filter(|num| {
            let diff = last_num.abs_diff(**num);
            if !(last_num > **num && diff >= 1 && diff <= 3) {
                last_num = **num;
                return true;
            }
            last_num = **num;
            false
        }).count();
        let mut last_num = report[0];
        let valid_greater = report[1..].iter().filter(|num| {
            let diff = last_num.abs_diff(**num);
            if !(last_num < **num && diff >= 1 && diff <= 3) {
                last_num = **num;
                return true;
            }
            last_num = **num;
            false
        }).count();
        valid_greater.min(valid_lesser) == 0
    }).count() as u32
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input.iter().filter(|report| {
        let mut last_num = u32::MAX;
        let mut skip_available = true;
        let skip_available_ref = &mut skip_available;
        let valid_lesser = report.iter().filter(|num| {
            let mut diff = last_num.abs_diff(**num);
            if last_num == u32::MAX {
                diff = 1;
            }
            if !(last_num > **num && diff >= 1 && diff <= 3) && *skip_available_ref {
                //last_num = **num;
                *skip_available_ref = false;
                return false;
            }
            if !(last_num > **num && diff >= 1 && diff <= 3) && !*skip_available_ref {
                last_num = **num;
                return true;
            }
            last_num = **num;
            false
        }).count();
        let mut last_num = u32::MIN;
        let mut skip_available = true;
        let skip_available_ref = &mut skip_available;
        let valid_greater = report.iter().filter(|num| {
            let mut diff = last_num.abs_diff(**num);
            if last_num == u32::MIN {
                diff = 1;
            }
            if !(last_num < **num && diff >= 1 && diff <= 3) && *skip_available_ref {
                //last_num = **num;
                *skip_available_ref = false;
                return false;
            }
            if !(last_num < **num && diff >= 1 && diff <= 3) && !*skip_available_ref {
                last_num = **num;
                return true;
            }
            last_num = **num;
            false
        }).count();
        if valid_greater.min(valid_lesser) != 0 {
            dbg!(report);
        }
        valid_greater.min(valid_lesser) == 0
    }).count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day02::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator("48 50 52 54 57 58 61");
        assert_eq!(solve_part1(&input), 2);
        assert_eq!(solve_part2(&input), 4);
    }
}