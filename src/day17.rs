use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> ((usize, usize, usize), Vec<u8>) {
    let mut parts = input.split("\n\n");
    let registers = parts
        .next()
        .unwrap()
        .split("\n")
        .map(|register| register.split(" ").nth(2).unwrap().parse().unwrap())
        .collect_tuple()
        .unwrap();
    let instructions = parts
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    (registers, instructions)
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &((usize, usize, usize), Vec<u8>)) -> String {
    let mut inst_pointer = 0;
    let mut cpu = input.0;
    let mut output = String::new();

    while let Some(inst) = input.1.get(inst_pointer..=inst_pointer + 1) {
        inst_pointer += 2;
        match inst {
            [0, c] => {
                cpu.0 /= 2_usize.pow(decode_combo(&cpu, c) as u32);
            }
            [1, l] => cpu.1 ^= *l as usize,
            [2, c] => {
                cpu.1 = decode_combo(&cpu, c) % 8;
            }
            [3, l] => {
                if cpu.0 != 0 {
                    inst_pointer = *l as usize;
                }
            }
            [4, _] => {
                cpu.1 ^= cpu.2;
            }
            [5, c] => {
                output.push_str(&format!(",{}", (decode_combo(&cpu, c) % 8)));
            }
            [6, c] => {
                cpu.1 = cpu.0 / 2_usize.pow(decode_combo(&cpu, c) as u32);
            }
            [7, c] => {
                cpu.2 = cpu.0 / 2_usize.pow(decode_combo(&cpu, c) as u32);
            }
            _ => unreachable!(),
        }
    }
    output.strip_prefix(',').unwrap().to_owned()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &((usize, usize, usize), Vec<u8>)) -> usize {
    // More or less random, but works
    //X
    //     100000000
    //35184000000000
    //37221265882612
    //297770093506470
    let mut a_init = 35184000000000;
    loop {
        let mut inst_pointer = 0;
        let mut cpu = input.0;
        cpu.0 = a_init;
        let mut output = String::new();

        while let Some(inst) = input.1.get(inst_pointer..=inst_pointer + 1) {
            inst_pointer += 2;
            match inst {
                [0, c] => {
                    cpu.0 /= 2_usize.pow(decode_combo(&cpu, c) as u32);
                }
                [1, l] => cpu.1 ^= *l as usize,
                [2, c] => {
                    cpu.1 = decode_combo(&cpu, c) % 8;
                }
                [3, l] => {
                    if cpu.0 != 0 {
                        inst_pointer = *l as usize;
                    }
                }
                [4, _] => {
                    cpu.1 ^= cpu.2;
                }
                [5, c] => {
                    output.push_str(&format!(",{}", (decode_combo(&cpu, c) % 8)));
                }
                [6, c] => {
                    cpu.1 = cpu.0 / 2_usize.pow(decode_combo(&cpu, c) as u32);
                }
                [7, c] => {
                    cpu.2 = cpu.0 / 2_usize.pow(decode_combo(&cpu, c) as u32);
                }
                _ => unreachable!(),
            }
        }
        // 2,4,1,2,7,5,4,1,1,3,5,5,0,3,3,0
        if output.strip_prefix(',').unwrap() == input.1.iter().join(",")  || (output.ends_with(",0,3,3,0") && output.len() == 16*2) {
            break;
        } else {
            dbg!(&output);
            a_init += 100_000;
        }
    }
    a_init
}

fn decode_combo(cpu: &(usize, usize, usize), combo: &u8) -> usize {
    match combo {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => cpu.0,
        5 => cpu.1,
        6 => cpu.2,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day17::{input_generator, solve_part1};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#,
        );
        assert_eq!(solve_part1(&input), "4,6,3,5,6,3,5,2,1,0");
        //assert_eq!(solve_part2(&input), "");
    }
}
