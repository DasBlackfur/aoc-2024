use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let map = input.split("\n\n").next().unwrap();
    let moves = input.split("\n\n").nth(1).unwrap();

    let map = map.split("\n").map(|line| line.chars().collect()).collect();

    let moves = moves
        .split("\n")
        .map(|line| line.chars().collect())
        .concat();

    (map, moves)
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &(Vec<Vec<char>>, Vec<char>)) -> usize {
    let mut map = input.0.clone();

    for robot_move in input.1.iter() {
        //for line in &map {
        //    println!("{}", String::from_iter(line));
        //}
        //println!("{}", robot_move);

        let robot_pos = find_robot(&map);
        match robot_move {
            '^' => {
                move_robot(&mut map, robot_pos, (0, -1));
            }
            '>' => {
                move_robot(&mut map, robot_pos, (1, 0));
            }
            'v' => {
                move_robot(&mut map, robot_pos, (0, 1));
            }
            '<' => {
                move_robot(&mut map, robot_pos, (-1, 0));
            }
            _ => unreachable!(),
        }
    }

    let boxes = find_boxes(&map);

    boxes.iter().map(|b| b.1 * 100 + b.0).sum()
}

fn find_robot(map: &[Vec<char>]) -> (usize, usize) {
    let (y, line) = map
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(&'@'))
        .unwrap();
    let x = line.iter().enumerate().find(|(_, c)| c == &&'@').unwrap().0;

    (x, y)
}

fn move_robot(map: &mut [Vec<char>], pos_now: (usize, usize), movement: (isize, isize)) {
    let in_front =
        map[(pos_now.1 as isize + movement.1) as usize][(pos_now.0 as isize + movement.0) as usize];

    if in_front == '#' {
    } else if in_front == 'O' {
        if let Some(free_pos) = find_free_space(map, pos_now, movement) {
            map[free_pos.1][free_pos.0] = 'O';
            map[pos_now.1][pos_now.0] = '.';
            map[(pos_now.1 as isize + movement.1) as usize]
                [(pos_now.0 as isize + movement.0) as usize] = '@';
        }
    } else {
        map[pos_now.1][pos_now.0] = '.';
        map[(pos_now.1 as isize + movement.1) as usize]
            [(pos_now.0 as isize + movement.0) as usize] = '@';
    }
}

fn find_free_space(
    map: &[Vec<char>],
    pos_now: (usize, usize),
    movement: (isize, isize),
) -> Option<(usize, usize)> {
    let mut step = 2;
    loop {
        let in_front = map[(pos_now.1 as isize + movement.1 * step) as usize]
            [(pos_now.0 as isize + movement.0 * step) as usize];

        if in_front == '#' {
            return None;
        } else if in_front == '.' {
            return Some((
                (pos_now.0 as isize + movement.0 * step) as usize,
                (pos_now.1 as isize + movement.1 * step) as usize,
            ));
        }
        step += 1;
    }
}

fn find_boxes(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'O')
                .map(|(x, _)| (x, y))
                .collect()
        })
        .concat()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &(Vec<Vec<char>>, Vec<char>)) -> usize {
    let mut map = scale_map(&input.0);

    for robot_move in input.1.iter() {
        //for line in &map {
        //    println!("{}", String::from_iter(line));
        //}
        //println!("{}", robot_move);

        let robot_pos = find_robot(&map);
        match robot_move {
            '^' => {
                move_robot_p2(&mut map, robot_pos, (0, -1));
            }
            '>' => {
                move_robot_p2(&mut map, robot_pos, (1, 0));
            }
            'v' => {
                move_robot_p2(&mut map, robot_pos, (0, 1));
            }
            '<' => {
                move_robot_p2(&mut map, robot_pos, (-1, 0));
            }
            _ => unreachable!(),
        }
    }

    let boxes = find_boxes_p2(&map);

    boxes.iter().map(|b| b.1 * 100 + b.0).sum()
}

fn scale_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    map.iter()
        .map(|line| {
            line.iter()
                .map(|c| match c {
                    '#' => vec!['#'; 2],
                    '@' => vec!['@', '.'],
                    '.' => vec!['.'; 2],
                    'O' => vec!['[', ']'],
                    _ => unreachable!(),
                })
                .concat()
        })
        .collect()
}

fn find_boxes_p2(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'[')
                .map(|(x, _)| (x, y))
                .collect()
        })
        .concat()
}

fn move_robot_p2(map: &mut [Vec<char>], pos_now: (usize, usize), movement: (isize, isize)) {
    let in_front =
        map[(pos_now.1 as isize + movement.1) as usize][(pos_now.0 as isize + movement.0) as usize];

    if in_front == '#' {
    } else if in_front == '[' || in_front == ']' {
        if has_free_space(map, pos_now, movement) {
            move_box(map, pos_now, movement);
            map[pos_now.1][pos_now.0] = '.';
            map[(pos_now.1 as isize + movement.1) as usize]
                [(pos_now.0 as isize + movement.0) as usize] = '@';
        }
    } else {
        map[pos_now.1][pos_now.0] = '.';
        map[(pos_now.1 as isize + movement.1) as usize]
            [(pos_now.0 as isize + movement.0) as usize] = '@';
    }
}

fn move_box(map: &mut [Vec<char>], pos_now: (usize, usize), movement: (isize, isize)) {
    if movement.0 != 0 {
        move_box_horizontal(map, pos_now, movement);
    } else {
        move_box_vertical(map, pos_now, movement, true);
    }
}

fn move_box_horizontal(map: &mut [Vec<char>], pos_now: (usize, usize), movement: (isize, isize)) {
    let pos_then = (
        ((pos_now.0 as isize + movement.0) as usize),
        ((pos_now.1 as isize + movement.1) as usize),
    );
    let c = map[pos_then.1][pos_then.0];
    let pos_target = (
        ((pos_then.0 as isize + movement.0) as usize),
        ((pos_then.1 as isize + movement.1) as usize),
    );
    if map[pos_target.1][pos_target.0] != '.' {
        move_box_horizontal(map, pos_then, movement);
    }
    map[pos_target.1][pos_target.0] = c;
}

fn move_box_vertical(map: &mut [Vec<char>], pos_now: (usize, usize), movement: (isize, isize), redirect: bool) {
    let pos_then = (
        ((pos_now.0 as isize + movement.0) as usize),
        ((pos_now.1 as isize + movement.1) as usize),
    );
    let c = map[pos_then.1][pos_then.0];
    let pos_target = (
        ((pos_then.0 as isize + movement.0) as usize),
        ((pos_then.1 as isize + movement.1) as usize),
    );
    if c == '[' && redirect {
        move_box_vertical(map, (pos_now.0 + 1, pos_now.1), movement, false);
    } else if c == ']' && redirect {
        move_box_vertical(map, (pos_now.0 - 1, pos_now.1), movement, false);
    }
    if map[pos_target.1][pos_target.0] != '.' {
        move_box_vertical(map, pos_then, movement, true);
    }
    map[pos_target.1][pos_target.0] = c;
    map[pos_then.1][pos_then.0] = '.';
}

fn has_free_space(map: &[Vec<char>], pos_now: (usize, usize), movement: (isize, isize)) -> bool {
    if movement.0 != 0 {
        free_space_horizontal(map, pos_now, movement)
    } else {
        free_space_vertical(map, pos_now, movement)
    }
}

fn free_space_vertical(
    map: &[Vec<char>],
    pos_now: (usize, usize),
    movement: (isize, isize),
) -> bool {
    let in_front =
        map[(pos_now.1 as isize + movement.1) as usize][(pos_now.0 as isize + movement.0) as usize];
    if in_front == '.' {
        true
    } else {
        let (mut left, mut right) = (false, false);

        if in_front == '[' {
            left = free_space_vertical(
                map,
                (
                    (pos_now.0 as isize + movement.0) as usize,
                    (pos_now.1 as isize + movement.1) as usize,
                ),
                movement,
            );
            right = free_space_vertical(
                map,
                (
                    (pos_now.0 as isize + movement.0 + 1) as usize,
                    (pos_now.1 as isize + movement.1) as usize,
                ),
                movement,
            );
        } else if in_front == ']' {
            left = free_space_vertical(
                map,
                (
                    (pos_now.0 as isize + movement.0 - 1) as usize,
                    (pos_now.1 as isize + movement.1) as usize,
                ),
                movement,
            );
            right = free_space_vertical(
                map,
                (
                    (pos_now.0 as isize + movement.0) as usize,
                    (pos_now.1 as isize + movement.1) as usize,
                ),
                movement,
            );
        }

        left && right
    }
}

fn free_space_horizontal(
    map: &[Vec<char>],
    pos_now: (usize, usize),
    movement: (isize, isize),
) -> bool {
    let mut step = 2;
    loop {
        let in_front = map[(pos_now.1 as isize + movement.1 * step) as usize]
            [(pos_now.0 as isize + movement.0 * step) as usize];

        if in_front == '#' {
            return false;
        } else if in_front == '.' {
            return true;
        }
        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#,
        );
        assert_eq!(solve_part1(&input), 10092);
        assert_eq!(solve_part2(&input), 9021);
    }
}
