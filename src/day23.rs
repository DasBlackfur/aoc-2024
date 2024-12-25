use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<[u8; 4]> {
    input.split("\n").map(|conn| conn.as_bytes()).map(|conn| [conn[0], conn[1], conn[3], conn[4]]).collect()
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[[u8; 4]]) -> usize {
    let t_s: Vec<[u8; 2]> = input.iter().flat_map(|conn| [[conn[0], conn[1]], [conn[2], conn[3]]]).unique().filter(|t| t.contains(&116)).collect();

    dbg!(t_s);
    
    1
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[[u8; 4]]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::day23::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#);
        assert_eq!(solve_part1(&input), 7);
        assert_eq!(solve_part2(&input), 0);
    }
}
