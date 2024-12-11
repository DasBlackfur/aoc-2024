use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[Vec<u8>]) -> usize {
    let mut zeros: Vec<(usize, usize)> = Vec::new();

    input.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, height)| {
            if *height == 0 {
                zeros.push((x, y));
            }
        })
    });

    zeros
        .iter()
        .map(|pos| check_neighbors(input, pos.0, pos.1).iter().unique().count())
        .sum()
}

fn check_neighbors(map: &[Vec<u8>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let search_height = map[x][y] + 1;

    let mut trail_count = Vec::new();

    if let Some(line) = map.get(x.wrapping_sub(1)) {
        let up = line[y];

        if up == search_height {
            if up == 9 {
                trail_count.push((x - 1, y));
            } else {
                trail_count.append(&mut check_neighbors(map, x - 1, y));
            }
        }
    }
    if let Some(line) = map.get(x + 1) {
        let down = line[y];

        if down == search_height {
            if down == 9 {
                trail_count.push((x + 1, y));
            } else {
                trail_count.append(&mut check_neighbors(map, x + 1, y));
            }
        }
    }
    if let Some(right) = map[x].get(y + 1) {
        if *right == search_height {
            if *right == 9 {
                trail_count.push((x, y + 1));
            } else {
                trail_count.append(&mut check_neighbors(map, x, y + 1));
            }
        }
    }
    if let Some(left) = map[x].get(y.wrapping_sub(1)) {
        if *left == search_height {
            if *left == 9 {
                trail_count.push((x, y - 1));
            } else {
                trail_count.append(&mut check_neighbors(map, x, y - 1));
            }
        }
    }
    //dbg!(&trail_count);

    trail_count
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[Vec<u8>]) -> u32 {
    let mut zeros: Vec<(usize, usize)> = Vec::new();

    input.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, height)| {
            if *height == 0 {
                zeros.push((x, y));
            }
        })
    });

    zeros
        .iter()
        .map(|pos| check_neighbors_part2(input, pos.0, pos.1))
        .sum()
}

fn check_neighbors_part2(map: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let search_height = map[x][y] + 1;

    let mut trail_count = 0;

    if let Some(line) = map.get(x.wrapping_sub(1)) {
        let up = line[y];

        if up == search_height {
            if up == 9 {
                trail_count += 1
            } else {
                trail_count += check_neighbors_part2(map, x - 1, y);
            }
        }
    }
    if let Some(line) = map.get(x + 1) {
        let down = line[y];

        if down == search_height {
            if down == 9 {
                trail_count += 1;
            } else {
                trail_count += check_neighbors_part2(map, x + 1, y);
            }
        }
    }
    if let Some(right) = map[x].get(y + 1) {
        if *right == search_height {
            if *right == 9 {
                trail_count += 1;
            } else {
                trail_count += check_neighbors_part2(map, x, y + 1);
            }
        }
    }
    if let Some(left) = map[x].get(y.wrapping_sub(1)) {
        if *left == search_height {
            if *left == 9 {
                trail_count += 1;
            } else {
                trail_count += check_neighbors_part2(map, x, y - 1);
            }
        }
    }
    //dbg!(&trail_count);

    trail_count
}

#[cfg(test)]
mod tests {
    use crate::day10::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#,
        );
        assert_eq!(solve_part1(&input), 36);
        assert_eq!(solve_part2(&input), 81);
    }
}
