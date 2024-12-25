use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
pub enum OP {
    And,
    Or,
    Xor,
}

type State = HashMap<String, Option<bool>>;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> (State, Vec<(String, String, String, OP)>) {
    let mut input = input.split("\n\n");
    let state = input
        .next()
        .unwrap()
        .split("\n")
        .map(|state_line| {
            (
                state_line[0..3].to_owned(),
                Some(state_line[5..].parse::<u8>().unwrap() != 0),
            )
        })
        .collect();
    let connections = input
        .next()
        .unwrap()
        .split("\n")
        .map(|conn_line| {
            let mut conn_line = conn_line.split_whitespace();
            let conn_a = conn_line.next().unwrap().to_owned();
            let op = match conn_line.next().unwrap() {
                "AND" => OP::And,
                "OR" => OP::Or,
                "XOR" => OP::Xor,
                _ => unreachable!(),
            };
            let conn_b = conn_line.next().unwrap().to_owned();
            conn_line.next();
            let conn_r = conn_line.next().unwrap().to_owned();
            (conn_a, conn_b, conn_r, op)
        })
        .collect();

    (state, connections)
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &(State, Vec<(String, String, String, OP)>)) -> usize {
    let mut state = input.0.clone();
    let mut conns: Vec<(String, String, String, OP)> = input.1.clone();
    let mut to_remove = Vec::new();

    while !conns.is_empty() {
        to_remove.clear();
        for (i, conn) in conns.iter().enumerate() {
            if let Some(x) = state.get(&conn.0).unwrap_or(&None) {
                if let Some(y) = state.get(&conn.1).unwrap_or(&None) {
                    let result = match conn.3 {
                        OP::And => x & y,
                        OP::Or => x | y,
                        OP::Xor => x ^ y,
                    };
                    state.insert(conn.2.clone(), Some(result));
                    to_remove.push(i);
                }
            }
        }
        conns = conns
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !to_remove.contains(i))
            .map(|(_, c)| c)
            .collect();
    }

    let mut num = 0;

    state
        .iter()
        .filter(|(key, _)| key.starts_with("z"))
        .for_each(|(key, value)| {
            let index: u8 = key[1..].parse().unwrap();
            num |= (value.unwrap() as usize) << index;
        });
    num
}

#[aoc(day24, part2)]
pub fn solve_part2(_input: &(State, Vec<(String, String, String, OP)>)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day24::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#,
        );
        assert_eq!(solve_part1(&input), 2024);
        assert_eq!(solve_part2(&input), 0);
    }
}
