use aoc_runner_derive::{aoc, aoc_generator};
use phf::{phf_map, Map};

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .map(|code| {
            let sequence = num_to_arrow(code);
            println!("{}", String::from_iter(&sequence));
            (
                sequence,
                String::from_iter(code.split_last().unwrap().1)
                    .parse::<usize>()
                    .unwrap(),
            )
        })
        .map(|(code, num)| {
            let sequence = arrow_to_arrow(&code);
            println!("{}", String::from_iter(&sequence));
            (sequence, num)
        })
        .map(|(code, num)| {
            let sequence = arrow_to_arrow(&code);
            println!("{}", String::from_iter(&sequence));
            (sequence, num)
        })
        .map(|(sequence, num)| {
            dbg!(sequence.len());
            dbg!(num);
            let prod = sequence.len() * num;
            dbg!(&prod);
            prod
        })
        .sum()
}

const KEYPAD_MAP: Map<char, (isize, isize)> = phf_map! {
    '7' => (0, 0),
    '8' => (1, 0),
    '9' => (2, 0),
    '4' => (0, 1),
    '5' => (1, 1),
    '6' => (2, 1),
    '1' => (0, 2),
    '2' => (1, 2),
    '3' => (2, 2),
    '0' => (1, 3),
    'A' => (2, 3)
};

fn num_to_arrow(code: &Vec<char>) -> Vec<char> {
    let mut current_pos: (isize, isize) = (2, 3);
    let mut sequence = Vec::new();
    let mut prev_y;

    for num in code {
        let target_pos = KEYPAD_MAP.get(num).unwrap();
        let last_char = *sequence.iter().rev().find(|c| c != &&'A').unwrap_or(&'^');
        dbg!(&last_char);
        prev_y = last_char == 'v' || last_char == '^';

        if current_pos.1 == 3 && target_pos.0 == 0 || prev_y {
            let y = target_pos.1 - current_pos.1;
            match y.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['^'; y.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['v'; y as usize]),
            }
            let x = target_pos.0 - current_pos.0;
            match x.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['<'; x.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['>'; x as usize]),
            }
        } else {
            let x = target_pos.0 - current_pos.0;
            match x.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['<'; x.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['>'; x as usize]),
            }
            let y = target_pos.1 - current_pos.1;
            match y.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['^'; y.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['v'; y as usize]),
            }
        }

        sequence.push('A');
        current_pos = *target_pos;
    }

    sequence
}

const REMOTE_MAP: Map<char, (isize, isize)> = phf_map! {
    '^' => (1, 0),
    'A' => (2, 0),
    '<' => (0, 1),
    'v' => (1, 1),
    '>' => (2, 1)
};

fn arrow_to_arrow(code: &Vec<char>) -> Vec<char> {
    let mut current_pos: (isize, isize) = (2, 0);
    let mut sequence = Vec::new();
    let mut prev_y = false;

    for num in code {
        let target_pos = REMOTE_MAP.get(num).unwrap();

        if current_pos.0 == 0 && target_pos.1 == 0 || prev_y { 
            prev_y = false;
            let y = target_pos.1 - current_pos.1;
            match y.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['^'; y.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['v'; y as usize]),
            }
            let x = target_pos.0 - current_pos.0;
            match x.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['<'; x.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['>'; x as usize]),
            }
        } else {
            prev_y = true;
            let x = target_pos.0 - current_pos.0;
            match x.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['<'; x.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['>'; x as usize]),
            }
            let y = target_pos.1 - current_pos.1;
            match y.cmp(&0) {
                std::cmp::Ordering::Less => sequence.append(&mut vec!['^'; y.unsigned_abs()]),
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => sequence.append(&mut vec!['v'; y as usize]),
            }
        }
        sequence.push('A');
        current_pos = *target_pos;
    }

    sequence
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day21::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(r#"375A"#);
        assert_eq!(solve_part1(&input), 126384);
        assert_eq!(solve_part2(&input), 0);
    }
}

/*
375A 70
^A<<^^Av>Avv>A
<A>Av<<AA>^AA>A<vA>A^A<vAA>A^A
<v<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<v<A>A>^AvA^A<A>A<v<A>A>^AAvA^A<A>A

^A<<^^A>vA>vvA
<A>A<<vAA^>AA>AvA<A>^AvA<AA>^A
<<vA^>>AvA^A<<vAA>A^>AA<A>vA^AAvA^Av<A>^Av<<A^>>AvA^<A>Av<A>^Av<<A^>>AAvA<^A>A
 */