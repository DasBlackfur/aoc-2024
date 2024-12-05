use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut input_parts = input.split("\n\n");
    let rules = input_parts.next().unwrap();
    let updates = input_parts.next().unwrap();

    let rules = rules.split("\n").map(|rule| {
        let mut rule_parts = rule.split("|");
        (rule_parts.next().unwrap().parse().unwrap(), rule_parts.next().unwrap().parse().unwrap())
    }).collect();

    let updates = updates.split("\n").map(|update| {
        update.split(",").map(|num| num.parse().unwrap()).collect()
    }).collect();

    (rules, updates)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    input.1.iter().filter(|update| {
        update.windows(2).filter(|update_part| {
            !input.0.contains(&(update_part[0], update_part[1]))
        }).count() == 0
    }).map(|valid_updates| {
        valid_updates[valid_updates.len() / 2]
    }).sum()
}

#[derive(Debug)]
enum Update {
    Branch(u32, Vec<Update>)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    input.1.iter().filter(|update| {
        update.windows(2).filter(|update_part| {
            !input.0.contains(&(update_part[0], update_part[1]))
        }).count() > 0
    }).map(|invalid_updates| {
        solve_tree(invalid_updates, &input.0)
    }).map(|valid_updates| {
        valid_updates[valid_updates.len() / 2]
    }).sum()
}

fn solve_tree(input: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    dbg!(input);
    let applying_rules = rules.iter().filter(|rule| input.contains(&rule.0) && input.contains(&rule.1)).collect_vec();

    let rule_tree = make_tree(None, &applying_rules, input.len());
    dbg!(find_longest_branch(&rule_tree, 0));

    Vec::new()
}

fn find_longest_branch(tree: &Vec<Update>, count: usize) -> Vec<Update> {
    tree.iter().map(|branch| find)
}

fn make_tree(constraint: Option<u32>, rules: &Vec<&(u32, u32)>, remaining_recursions: usize) -> Vec<Update> {
    rules.iter().filter(|rule| {
        if let Some(must_num) = constraint {
            return rule.0 == must_num
        }
        true
    }).map(|rule| {
        if remaining_recursions == 0 {
            return Update::Branch(rule.0, vec![])
        }
        Update::Branch(rule.0, make_tree(Some(rule.1), rules, remaining_recursions - 1))
    }).collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::day05::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#);
        assert_eq!(solve_part1(&input), 143);
        assert_eq!(solve_part2(&input), 123);
    }
}