use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let mut path: HashMap<(usize, usize), usize> = HashMap::new();
    let mut shortcuts: Vec<usize> = Vec::new();

    let start_line = input
        .iter()
        .enumerate()
        .find(|line| line.1.contains(&'S'))
        .unwrap();
    let start = (
        start_line
            .1
            .iter()
            .enumerate()
            .find(|(_, c)| c == &&'S')
            .unwrap()
            .0,
        start_line.0,
    );
    let end_line = input
        .iter()
        .enumerate()
        .find(|line| line.1.contains(&'E'))
        .unwrap();
    let end = (
        end_line
            .1
            .iter()
            .enumerate()
            .find(|(_, c)| c == &&'E')
            .unwrap()
            .0,
        end_line.0,
    );

    step(input, &mut path, start, end);

    for step in path.iter() {
        find_shortcut(step, &path, &mut shortcuts);
    }

    shortcuts.len()
}

fn step(
    map: &[Vec<char>],
    path: &mut HashMap<(usize, usize), usize>,
    start: (usize, usize),
    end: (usize, usize),
) {
    let mut pos = start;
    let mut prev_pos = start;
    let mut score = 0;

    loop {
        path.insert(pos, score);
        score += 1;
        if pos == end {
            return;
        }
        let new_pos = get_next_pos(map, pos, prev_pos);
        prev_pos = pos;
        pos = new_pos;
    }
}

fn get_next_pos(
    map: &[Vec<char>],
    pos: (usize, usize),
    prev_pos: (usize, usize),
) -> (usize, usize) {
    let next_pos = (pos.0.wrapping_sub(1), pos.1);
    if next_pos != prev_pos {
        if let Some(line) = map.get(next_pos.1) {
            if let Some(c) = line.get(next_pos.0) {
                if *c == '.' || *c == 'E' {
                    return next_pos;
                }
            }
        }
    }
    let next_pos = (pos.0 + 1, pos.1);
    if next_pos != prev_pos {
        if let Some(line) = map.get(next_pos.1) {
            if let Some(c) = line.get(next_pos.0) {
                if *c == '.' || *c == 'E' {
                    return next_pos;
                }
            }
        }
    }
    let next_pos = (pos.0, pos.1.wrapping_sub(1));
    if next_pos != prev_pos {
        if let Some(line) = map.get(next_pos.1) {
            if let Some(c) = line.get(next_pos.0) {
                if *c == '.' || *c == 'E' {
                    return next_pos;
                }
            }
        }
    }
    let next_pos = (pos.0, pos.1 + 1);
    if next_pos != prev_pos {
        if let Some(line) = map.get(next_pos.1) {
            if let Some(c) = line.get(next_pos.0) {
                if *c == '.' || *c == 'E' {
                    return next_pos;
                }
            }
        }
    }

    unreachable!()
}

fn find_shortcut(
    step: (&(usize, usize), &usize),
    path: &HashMap<(usize, usize), usize>,
    shortcuts: &mut Vec<usize>,
) {
    let cut_pos = (step.0 .0.wrapping_sub(2), step.0 .1);
    if let Some(cut) = path.get(&cut_pos) {
        let cut_length = *cut as isize - *step.1 as isize - 2;
        if cut_length > 99 {
            shortcuts.push(cut_length as usize);
        }
    }
    let cut_pos = (step.0 .0, step.0 .1.wrapping_sub(2));
    if let Some(cut) = path.get(&cut_pos) {
        let cut_length = *cut as isize - *step.1 as isize - 2;
        if cut_length > 99 {
            shortcuts.push(cut_length as usize);
        }
    }
    let cut_pos = (step.0 .0 + 2, step.0 .1);
    if let Some(cut) = path.get(&cut_pos) {
        let cut_length = *cut as isize - *step.1 as isize - 2;
        if cut_length > 99 {
            shortcuts.push(cut_length as usize);
        }
    }
    let cut_pos = (step.0 .0, step.0 .1 + 2);
    if let Some(cut) = path.get(&cut_pos) {
        let cut_length = *cut as isize - *step.1 as isize - 2;
        if cut_length > 99 {
            shortcuts.push(cut_length as usize);
        }
    }
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    let mut path: HashMap<(usize, usize), usize> = HashMap::new();

    let start_line = input
        .iter()
        .enumerate()
        .find(|line| line.1.contains(&'S'))
        .unwrap();
    let start = (
        start_line
            .1
            .iter()
            .enumerate()
            .find(|(_, c)| c == &&'S')
            .unwrap()
            .0,
        start_line.0,
    );
    let end_line = input
        .iter()
        .enumerate()
        .find(|line| line.1.contains(&'E'))
        .unwrap();
    let end = (
        end_line
            .1
            .iter()
            .enumerate()
            .find(|(_, c)| c == &&'E')
            .unwrap()
            .0,
        end_line.0,
    );

    step(input, &mut path, start, end);

    path.iter().map(|step| find_shortcut_p2(step, &path)).sum()
}

fn find_shortcut_p2(
    step: (&(usize, usize), &usize),
    path: &HashMap<(usize, usize), usize>,
) -> usize {
    path.iter()
        .filter_map(|(cut_pos, cut_len)| {
            let diff = cut_pos.0.abs_diff(step.0 .0) + cut_pos.1.abs_diff(step.0 .1);
            if diff <= 20 {
                Some((cut_pos, cut_len, diff))
            } else {
                None
            }
        })
        .filter(|(_, cut_len, diff)| **cut_len as isize - *step.1 as isize - *diff as isize > 99)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day20::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#,
        );
        assert_eq!(solve_part1(&input), 0);
        assert_eq!(solve_part2(&input), 0);
    }
}
