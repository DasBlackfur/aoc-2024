use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Machine {
    prize_x: isize,
    prize_y: isize,
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Machine> {
    let machine_regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    input
        .split("\n\n")
        .map(|machine| {
            let captures = machine_regex.captures(machine).unwrap().extract::<6>().1;
            Machine {
                a_x: captures[0].parse().unwrap(),
                a_y: captures[1].parse().unwrap(),
                b_x: captures[2].parse().unwrap(),
                b_y: captures[3].parse().unwrap(),
                prize_x: captures[4].parse().unwrap(),
                prize_y: captures[5].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Machine]) -> isize {
    input
        .iter()
        .filter_map(|machine| {
            let x = (-(machine.b_x * machine.prize_y - machine.b_y * machine.prize_x))
                / (machine.a_x * machine.b_y - machine.a_y * machine.b_x);
            let y = (machine.a_x * machine.prize_y - machine.a_y * machine.prize_x)
                / (machine.a_x * machine.b_y - machine.a_y * machine.b_x);

            if (x * machine.a_x + y * machine.b_x) == machine.prize_x
                && (x * machine.a_y + y * machine.b_y) == machine.prize_y
            {
                return Some(x * 3 + y);
            }
            None
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[Machine]) -> isize {
    input
        .iter()
        .map(|machine| Machine {
            prize_x: machine.prize_x + 10000000000000,
            prize_y: machine.prize_y + 10000000000000,
            a_x: machine.a_x,
            a_y: machine.a_y,
            b_x: machine.b_x,
            b_y: machine.b_y,
        })
        .filter_map(|machine| {
            let x = (-(machine.b_x * machine.prize_y - machine.b_y * machine.prize_x))
                / (machine.a_x * machine.b_y - machine.a_y * machine.b_x);
            let y = (machine.a_x * machine.prize_y - machine.a_y * machine.prize_x)
                / (machine.a_x * machine.b_y - machine.a_y * machine.b_x);

            if (x * machine.a_x + y * machine.b_x) == machine.prize_x
                && (x * machine.a_y + y * machine.b_y) == machine.prize_y
            {
                return Some(x * 3 + y);
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day13::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#,
        );
        assert_eq!(solve_part1(&input), 480);
        assert_eq!(solve_part2(&input), 875318608908);
    }
}
