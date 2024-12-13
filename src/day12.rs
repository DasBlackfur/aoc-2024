use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|line| line.chars().collect()).collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let mut regions: HashSet<(usize, usize)> = HashSet::new();

    input.iter().enumerate().map(|(x, line)| {
        line.iter().enumerate().map(|(y, c)| {
            let result = check_directions(&mut regions, input, x, y, c, (0, 0));
            result.0 * result.1
        }).sum::<usize>()
    }).sum()
}

fn check_directions(regions: &mut HashSet<(usize, usize)>, input: &[Vec<char>], x: usize, y: usize, c: &char, mut prev: (usize, usize)) -> (usize, usize) {
    if regions.contains(&(x, y)) {
        return prev;
    }

    regions.insert((x, y));
    prev.0 += 1;

    if let Some(up_c) = input.get(x.wrapping_sub(1)) {
        if &up_c[y] != c {
            prev.1 += 1;
        } else {
            prev = check_directions(regions, input, x -1, y, c, prev);
        }
    } else {
        prev.1 += 1
    }
    if let Some(left_c) = input[x].get(y.wrapping_sub(1)) {
        if left_c != c {
            prev.1 += 1;
        } else {
            prev = check_directions(regions, input, x, y -1, c, prev);
        }
    } else {
        prev.1 += 1
    }
    if let Some(down_c) = input.get(x +1) {
        if &down_c[y] != c {
            prev.1 += 1;
        } else {
            prev = check_directions(regions, input, x +1, y, c, prev);
        }
    } else {
        prev.1 += 1
    }
    if let Some(right_c) = input[x].get(y +1) {
        if right_c != c {
            prev.1 += 1;
        } else {
            prev = check_directions(regions, input, x, y +1, c, prev);
        }
    } else {
        prev.1 += 1
    }

    prev
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    let mut regions: HashSet<(usize, usize)> = HashSet::new();

    input.iter().enumerate().map(|(x, line)| {
        line.iter().enumerate().map(|(y, c)| {
            let result = check_corners(&mut regions, input, x, y, c, (0, 0));

            result.0 * (result.1)
        }).sum::<usize>()
    }).sum()
}

fn check_corners(regions: &mut HashSet<(usize, usize)>, input: &[Vec<char>], x: usize, y: usize, c: &char, mut prev: (usize, usize)) -> (usize, usize) {
    if regions.contains(&(x, y)) {
        return prev;
    }

    regions.insert((x, y));
    prev.0 += 1;


    let surrounding: Vec<char> = (0..3).map(|x_o| (0..3).map(move |y_o| {
        if let Some(line) = input.get((x + 1).wrapping_sub(x_o)) {
            if let Some(c_o) = line.get((y + 1).wrapping_sub(y_o)) {
                *c_o
            } else {
                '_'
            }
        } else {
            '_'
        }
    }).rev().collect()).rev().concat();

    if &surrounding[0] != c && &surrounding[1] == c && &surrounding[3] == c {
        prev.1 += 1
    }
    if &surrounding[2] != c && &surrounding[1] == c && &surrounding[5] == c {
        prev.1 += 1
    }
    if &surrounding[8] != c && &surrounding[5] == c && &surrounding[7] == c {
        prev.1 += 1
    }
    if &surrounding[6] != c && &surrounding[7] == c && &surrounding[3] == c {
        prev.1 += 1
    }

    if &surrounding[1] != c && &surrounding[3] != c {
        prev.1 += 1
    }
    if &surrounding[1] != c && &surrounding[5] != c {
        prev.1 += 1
    }
    if &surrounding[5] != c && &surrounding[7] != c {
        prev.1 += 1
    }
    if &surrounding[7] != c && &surrounding[3] != c {
        prev.1 += 1
    }


    if let Some(up_c) = input.get(x.wrapping_sub(1)) {
        if &up_c[y] == c {
            prev = check_corners(regions, input, x -1, y, c, prev);
        }
    }
    if let Some(left_c) = input[x].get(y.wrapping_sub(1)) {
        if left_c == c {
            prev = check_corners(regions, input, x, y -1, c, prev);
        }
    }
    if let Some(down_c) = input.get(x +1) {
        if &down_c[y] == c {
            prev = check_corners(regions, input, x +1, y, c, prev);
        }
    }
    if let Some(right_c) = input[x].get(y +1) {
        if right_c == c {
            prev = check_corners(regions, input, x, y +1, c, prev);
        }
    }

    prev
}

#[cfg(test)]
mod tests {
    use crate::day12::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#);
        assert_eq!(solve_part1(&input), 1930);
        assert_eq!(solve_part2(&input), 1206);
    }
}
