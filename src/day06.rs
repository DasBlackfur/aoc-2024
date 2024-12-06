use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> (Vec<Vec<bool>>, (usize, usize, u8)) {
    let mut guard = (0, 0, 0);
    let guard_ref = &mut guard;
    (
        input
            .split("\n")
            .enumerate()
            .map(|line| {
                line.1
                    .chars()
                    .enumerate()
                    .map(|cell| match cell.1 {
                        '.' => false,
                        '#' => true,
                        '^' => {
                            *guard_ref = (line.0, cell.0, 0);
                            false
                        }
                        '>' => {
                            *guard_ref = (line.0, cell.0, 1);
                            false
                        }
                        'v' => {
                            *guard_ref = (line.0, cell.0, 2);
                            false
                        }
                        '<' => {
                            *guard_ref = (line.0, cell.0, 3);
                            false
                        }
                        _ => panic!("invalid input in cell"),
                    })
                    .collect()
            })
            .collect(),
        guard,
    )
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &(Vec<Vec<bool>>, (usize, usize, u8))) -> u32 {
    let mut guard = (input.1 .0 as isize, input.1 .1 as isize, input.1 .2);
    let mut positions = Vec::new();
    let mut end = false;
    let mut count = 0;
    while !end {
        match guard.2 {
            0 => {
                let x = guard.0.wrapping_sub(1);
                if x < 0 {
                    end = true;
                } else {
                    match input.0.get(x as usize) {
                        Some(line) => match line.get(guard.1 as usize) {
                            Some(cell) => {
                                if *cell {
                                    guard.2 = (guard.2 + 1) % 4;
                                } else {
                                    guard.0 = x;
                                    positions.push((guard.0, guard.1));
                                    count += 1;
                                }
                            }
                            None => end = true,
                        },
                        None => end = true,
                    }
                }
            }
            1 => {
                let y = guard.1.wrapping_add(1);
                if y < 0 {
                    end = true;
                } else {
                    match input.0.get(guard.0 as usize) {
                        Some(line) => match line.get(y as usize) {
                            Some(cell) => {
                                if *cell {
                                    guard.2 = (guard.2 + 1) % 4;
                                } else {
                                    guard.1 = y;
                                    positions.push((guard.0, guard.1));
                                    count += 1;
                                }
                            }
                            None => end = true,
                        },
                        None => end = true,
                    }
                }
            },
            2 => {
                let x = guard.0.wrapping_add(1);
                if x < 0 {
                    end = true;
                } else {
                    match input.0.get(x as usize) {
                        Some(line) => match line.get(guard.1 as usize) {
                            Some(cell) => {
                                if *cell {
                                    guard.2 = (guard.2 + 1) % 4;
                                } else {
                                    guard.0 = x;
                                    positions.push((guard.0, guard.1));
                                    count += 1;
                                }
                            }
                            None => end = true,
                        },
                        None => end = true,
                    }
                }
            }
            3 => {
                let y = guard.1.wrapping_sub(1);
                if y < 0 {
                    end = true;
                } else {
                    match input.0.get(guard.0 as usize) {
                        Some(line) => match line.get(y as usize) {
                            Some(cell) => {
                                if *cell {
                                    guard.2 = (guard.2 + 1) % 4;
                                } else {
                                    guard.1 = y;
                                    positions.push((guard.0, guard.1));
                                    count += 1;
                                }
                            }
                            None => end = true,
                        },
                        None => end = true,
                    }
                }
            }
            _ => {}
        };
        if count > 1000000 {
            end = true;
        }
    }
    if count > 1000000 {
        return u32::MAX;
    }
    positions.iter().unique().count() as u32
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &(Vec<Vec<bool>>, (usize, usize, u8))) -> u32 {
    (0..input.0.len()).into_par_iter().map(|x| (0..input.0[0].len()).map(move |y| {
        if (x, y) != (input.1.0, input.1.1) && !input.0[x][y] {
            let mut modified_input = input.clone();
            modified_input.0[x][y] = true;
            if solve_part1(&modified_input) == u32::MAX {
                return 1_u32;
            }
        }
        0_u32
    }).sum::<u32>()).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::day06::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#,
        );
        assert_eq!(solve_part1(&input), 41);
        assert_eq!(solve_part2(&input), 6);
    }
}
