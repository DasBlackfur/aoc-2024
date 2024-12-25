use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Vec<(bool, [u8; 5])> {
    input
        .split("\n\n")
        .map(|schematic| {
            let mut spec = [0; 5];
            let mut lines = schematic.split("\n");
            let key = lines.next().unwrap() == ".....";
            lines.take(5).for_each(|line| {
                line.char_indices()
                    .for_each(|(i, c)| spec[i] += if c == '#' { 1 } else { 0 });
            });
            (key, spec)
        })
        .collect()
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &[(bool, [u8; 5])]) -> usize {
    input
        .iter()
        .filter(|(key, _)| *key)
        .map(|(_, key)| {
            let sum: usize = input
                .iter()
                .filter(|(key, _)| !key)
                .map(|(_, lock)| {
                    if key[0] + lock[0] <= 5
                        && key[1] + lock[1] <= 5
                        && key[2] + lock[2] <= 5
                        && key[3] + lock[3] <= 5
                        && key[4] + lock[4] <= 5
                    {
                        1_usize
                    } else {
                        0_usize
                    }
                })
                .sum();

            sum
        })
        .sum()
}

#[aoc(day25, part2)]
pub fn solve_part2(input: &[(bool, [u8; 5])]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day25::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#,
        );
        assert_eq!(solve_part1(&input), 3);
        assert_eq!(solve_part2(&input), 0);
    }
}
