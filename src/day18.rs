use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<(usize, usize)> {
    input
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[(usize, usize)]) -> usize {
    let mut memory = vec![vec![false; 71]; 71];

    input.iter().take(1024).for_each(|pos| {
        memory[pos.1][pos.0] = true;
    });

    let mut positions: HashMap<(usize, usize), usize> = HashMap::new();

    step(&memory, (0, 0), &mut positions, 0)
}

fn step(
    memory: &[Vec<bool>],
    pos: (usize, usize),
    positions: &mut HashMap<(usize, usize), usize>,
    score: usize,
) -> usize {
    if memory[pos.1][pos.0] {
        usize::MAX
    } else if pos == (70, 70) {
        return score;
    } else if positions.get(&pos).is_some_and(|s| &score >= s) {
        return usize::MAX;
    } else {
        positions.insert(pos, score);
        let (mut left, mut up, mut right, mut down) =
            (usize::MAX, usize::MAX, usize::MAX, usize::MAX);
        if let Some(left_pos) = make_pos(pos, (-1, 0)) {
            left = step(memory, left_pos, positions, score + 1);
        }
        if let Some(up_pos) = make_pos(pos, (0, -1)) {
            up = step(memory, up_pos, positions, score + 1);
        }
        if let Some(right_pos) = make_pos(pos, (1, 0)) {
            right = step(memory, right_pos, positions, score + 1);
        }
        if let Some(down_pos) = make_pos(pos, (0, 1)) {
            down = step(memory, down_pos, positions, score + 1);
        }

        left.min(up).min(right).min(down)
    }
}

fn make_pos(pos: (usize, usize), movement: (isize, isize)) -> Option<(usize, usize)> {
    let x = pos.0 as isize + movement.0;
    if !(0..=70).contains(&x) {
        return None;
    }
    let y = pos.1 as isize + movement.1;
    if !(0..=70).contains(&y) {
        return None;
    }
    Some((x as usize, y as usize))
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[(usize, usize)]) -> String {
    let mut memory = vec![vec![false; 71]; 71];

    // Binary search by hand :3333
    input.iter().take(2500).for_each(|pos| {
        memory[pos.1][pos.0] = true;
    });

    let mut positions: HashMap<(usize, usize), usize> = HashMap::new();
    let mut next_bytes = input.iter().enumerate().skip(2499);

    loop {
        let next_pos = next_bytes.next().unwrap();
        //dbg!(next_pos.0);
        memory[next_pos.1 .1][next_pos.1 .0] = true;
        let steps = step(&memory, (0, 0), &mut positions, 0);
        positions.clear();

        if steps == usize::MAX {
            return format!("{},{}", next_pos.1 .0, next_pos.1 .1);
        }
    }
}
