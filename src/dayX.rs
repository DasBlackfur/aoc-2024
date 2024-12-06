use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> () {
}

#[aoc(dayX, part1)]
pub fn solve_part1(input: &()) -> u32 {
    0
}

#[aoc(dayX, part2)]
pub fn solve_part2(input: &()) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::dayX::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator("");
        assert_eq!(solve_part1(&input), 0);
        assert_eq!(solve_part2(&input), 0);
    }
}
