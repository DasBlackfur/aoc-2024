use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u32 {
    let mut sum: u32 = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if let Some(count) = check_star(x, y, input) {
                sum += count;
            }
        }
    }
    sum
}

fn check_star(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<u32> {
    if input.get(start_x)?.get(start_y)? == &'X' {
        let backwards_horizontal =
            check_backwards_horizontal(start_x, start_y, input).unwrap_or(false);
        let backwards_vertical = check_backwards_vertical(start_x, start_y, input).unwrap_or(false);
        let backwards_diagonal_up =
            check_backwards_diagonal_up(start_x, start_y, input).unwrap_or(false);
        let backwards_diagonal_down =
            check_backwards_diagonal_down(start_x, start_y, input).unwrap_or(false);
        let forwards_horizontal =
            check_forwards_horizontal(start_x, start_y, input).unwrap_or(false);
        let forwards_vertical = check_forwards_vertical(start_x, start_y, input).unwrap_or(false);
        let forwards_diagonal_up =
            check_forwards_diagonal_up(start_x, start_y, input).unwrap_or(false);
        let forwards_diagonal_down =
            check_forwards_diagonal_down(start_x, start_y, input).unwrap_or(false);
        return Some(
            backwards_horizontal as u32
                + backwards_vertical as u32
                + backwards_diagonal_up as u32
                + backwards_diagonal_down as u32
                + forwards_horizontal as u32
                + forwards_vertical as u32
                + forwards_diagonal_up as u32
                + forwards_diagonal_down as u32,
        );
    }
    None
}

fn check_backwards_horizontal(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input.get(start_x)?.get(start_y.wrapping_sub(1))? == &'M';
    let b = input.get(start_x)?.get(start_y.wrapping_sub(2))? == &'A';
    let c = input.get(start_x)?.get(start_y.wrapping_sub(3))? == &'S';
    Some(a && b && c)
}

fn check_backwards_vertical(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input.get(start_x.wrapping_sub(1))?.get(start_y)? == &'M';
    let b = input.get(start_x.wrapping_sub(2))?.get(start_y)? == &'A';
    let c = input.get(start_x.wrapping_sub(3))?.get(start_y)? == &'S';
    Some(a && b && c)
}
fn check_backwards_diagonal_up(
    start_x: usize,
    start_y: usize,
    input: &[Vec<char>],
) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_sub(1))?
        .get(start_y.wrapping_sub(1))?
        == &'M';
    let b = input
        .get(start_x.wrapping_sub(2))?
        .get(start_y.wrapping_sub(2))?
        == &'A';
    let c = input
        .get(start_x.wrapping_sub(3))?
        .get(start_y.wrapping_sub(3))?
        == &'S';
    Some(a && b && c)
}
fn check_backwards_diagonal_down(
    start_x: usize,
    start_y: usize,
    input: &[Vec<char>],
) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_add(1))?
        .get(start_y.wrapping_sub(1))?
        == &'M';
    let b = input
        .get(start_x.wrapping_add(2))?
        .get(start_y.wrapping_sub(2))?
        == &'A';
    let c = input
        .get(start_x.wrapping_add(3))?
        .get(start_y.wrapping_sub(3))?
        == &'S';
    Some(a && b && c)
}
fn check_forwards_horizontal(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input.get(start_x)?.get(start_y.wrapping_add(1))? == &'M';
    let b = input.get(start_x)?.get(start_y.wrapping_add(2))? == &'A';
    let c = input.get(start_x)?.get(start_y.wrapping_add(3))? == &'S';
    Some(a && b && c)
}

fn check_forwards_vertical(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input.get(start_x.wrapping_add(1))?.get(start_y)? == &'M';
    let b = input.get(start_x.wrapping_add(2))?.get(start_y)? == &'A';
    let c = input.get(start_x.wrapping_add(3))?.get(start_y)? == &'S';
    Some(a && b && c)
}
fn check_forwards_diagonal_up(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_add(1))?
        .get(start_y.wrapping_add(1))?
        == &'M';
    let b = input
        .get(start_x.wrapping_add(2))?
        .get(start_y.wrapping_add(2))?
        == &'A';
    let c = input
        .get(start_x.wrapping_add(3))?
        .get(start_y.wrapping_add(3))?
        == &'S';
    Some(a && b && c)
}
fn check_forwards_diagonal_down(
    start_x: usize,
    start_y: usize,
    input: &[Vec<char>],
) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_sub(1))?
        .get(start_y.wrapping_add(1))?
        == &'M';
    let b = input
        .get(start_x.wrapping_sub(2))?
        .get(start_y.wrapping_add(2))?
        == &'A';
    let c = input
        .get(start_x.wrapping_sub(3))?
        .get(start_y.wrapping_add(3))?
        == &'S';
    Some(a && b && c)
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u32 {
    let mut sum: u32 = 0;
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if let Some(count) = check_x(x, y, input) {
                sum += count;
            }
        }
    }
    sum
}

fn check_x(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<u32> {
    if input.get(start_x)?.get(start_y)? == &'A' {
        let one = check_x_down_left_up_right(start_x, start_y, input).unwrap_or(false)
            || check_x_up_right_down_left(start_x, start_y, input).unwrap_or(false);
        let two = check_x_down_right_up_left(start_x, start_y, input).unwrap_or(false)
            || check_x_up_left_down_right(start_x, start_y, input).unwrap_or(false);
        return Some((one && two) as u32);
    }
    None
}
fn check_x_up_left_down_right(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_sub(1))?
        .get(start_y.wrapping_sub(1))?
        == &'M';
    let b = input
        .get(start_x.wrapping_add(1))?
        .get(start_y.wrapping_add(1))?
        == &'S';
    Some(a && b)
}

fn check_x_down_right_up_left(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_sub(1))?
        .get(start_y.wrapping_sub(1))?
        == &'S';
    let b = input
        .get(start_x.wrapping_add(1))?
        .get(start_y.wrapping_add(1))?
        == &'M';
    Some(a && b)
}

fn check_x_up_right_down_left(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_sub(1))?
        .get(start_y.wrapping_add(1))?
        == &'M';
    let b = input
        .get(start_x.wrapping_add(1))?
        .get(start_y.wrapping_sub(1))?
        == &'S';
    Some(a && b)
}

fn check_x_down_left_up_right(start_x: usize, start_y: usize, input: &[Vec<char>]) -> Option<bool> {
    let a = input
        .get(start_x.wrapping_sub(1))?
        .get(start_y.wrapping_add(1))?
        == &'S';
    let b = input
        .get(start_x.wrapping_add(1))?
        .get(start_y.wrapping_sub(1))?
        == &'M';
    Some(a && b)
}

#[cfg(test)]
mod tests {
    use crate::day04::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
        );
        assert_eq!(solve_part1(&input), 18);
        assert_eq!(solve_part2(&input), 9);
    }
}
