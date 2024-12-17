use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|line| line.chars().collect()).collect()
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    let input = input.to_vec();
    let mut positions: HashSet<(u8, usize, usize)> = HashSet::new();
    let start_line = input.iter().enumerate().find(|line| line.1.contains(&'S')).unwrap();
    let start = (start_line.1.iter().enumerate().find(|(_, c)| c == &&'S').unwrap().0, start_line.0);

    let end_line = input.iter().enumerate().find(|line| line.1.contains(&'E')).unwrap();
    let end = (end_line.1.iter().enumerate().find(|(_, c)| c == &&'E').unwrap().0, end_line.0);

    std::thread::Builder::new().stack_size(size_of::<usize>() * 1_000_000).spawn(move || {
        check_tile(&input, 2, &start, 0, &end, &mut positions)
    }).unwrap().join().unwrap()
}

fn check_tile(map: &[Vec<char>], direction: u8, pos: &(usize, usize), score: usize, goal: &(usize, usize), positions: &mut HashSet<(u8, usize, usize)>) -> usize {
    if map[pos.1][pos.0] == '#' {
        return usize::MAX;
    } else if pos == goal {
        return score;
    }
    if positions.get(&(direction, pos.0, pos.1)).is_some() {
        return usize::MAX;
    } else {
        positions.insert((direction, pos.0, pos.1));
    }
    if score == 0 {
        let left = check_tile(map, 0, &(pos.0 -1, pos.1), 2001, goal, positions);
        let up = check_tile(map,1, &(pos.0, pos.1 -1), 1001, goal, positions);
        let right = check_tile(map, 2, &(pos.0 +1, pos.1), 1, goal, positions);
        let down = check_tile(map, 3, &(pos.0, pos.1 +1), 1001, goal, positions);

        left.min(up).min(right).min(down)
    } else {
        let mut left = usize::MAX;
        let mut up = usize::MAX;
        let mut right = usize::MAX;
        let mut down = usize::MAX;
        if direction != 2 {
            left = check_tile(map, 0, &(pos.0 -1, pos.1), score  + if direction == 0 {1} else {1001}, goal, positions);
        }
        if direction != 3 {
            up = check_tile(map,1, &(pos.0, pos.1 -1), score  + if direction == 1 {1} else {1001}, goal, positions);
        }
        if direction != 0 {
            right = check_tile(map, 2, &(pos.0 +1, pos.1), score + if direction == 2 {1} else {1001}, goal, positions);
        }
        if direction != 1 {
            down = check_tile(map, 3, &(pos.0, pos.1 +1), score + if direction == 3 {1} else {1001}, goal, positions);
        }

        //positions.remove(&(direction, pos.0, pos.1));

        left.min(up).min(right).min(down)
    }
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day16::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#);
        assert_eq!(solve_part1(&input), 7036);
        assert_eq!(solve_part2(&input), 0);
    }
}
