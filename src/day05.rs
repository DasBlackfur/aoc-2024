use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let input = input.replace("\r", "");
    let mut input_parts = input.split("\n\n");
    let rules = input_parts.next().unwrap();
    let updates = input_parts.next().unwrap();

    let rules = rules
        .split("\n")
        .map(|rule| {
            let mut rule_parts = rule.split("|");
            (
                rule_parts.next().unwrap().parse().unwrap(),
                rule_parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let updates = updates
        .split("\n")
        .map(|update| update.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    input
        .1
        .iter()
        .filter(|update| {
            update
                .windows(2)
                .filter(|update_part| !input.0.contains(&(update_part[0], update_part[1])))
                .count()
                == 0
        })
        .map(|valid_updates| valid_updates[valid_updates.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> u32 {
    input
        .1
        .iter()
        .filter(|update| {
            update
                .windows(2)
                .filter(|update_part| !input.0.contains(&(update_part[0], update_part[1])))
                .count()
                > 0
        })
        .map(|invalid_updates| {
            let mut modified_update = invalid_updates.clone();
            modified_update.sort_by(|a, b| {
                if let Some(valid_rule) = input
                    .0
                    .iter()
                    .find(|rule| (&rule.0 == a && &rule.1 == b) || (&rule.0 == b && &rule.1 == a))
                {
                    if &(*a, *b) == valid_rule {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                }
                std::cmp::Ordering::Equal
            });
            //dbg!(modified_update);
            modified_update
        })
        .map(|valid_updates| valid_updates[valid_updates.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day05::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"47|53
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
97,13,75,29,47"#,
        );
        assert_eq!(solve_part1(&input), 143);
        assert_eq!(solve_part2(&input), 123);
    }
}
