use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut parts = input.split("\n\n");

    let patterns = parts
        .next()
        .unwrap()
        .split(", ")
        .map(|pattern| pattern.as_bytes().to_vec())
        .collect();

    let designs = parts
        .next()
        .unwrap()
        .split("\n")
        .map(|design| design.as_bytes().to_vec())
        .collect();

    (patterns, designs)
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &(Vec<Vec<u8>>, Vec<Vec<u8>>)) -> usize {
    let mut pattern_cache: HashMap<Vec<u8>, bool> = HashMap::new();
    input
        .1
        .iter()
        .filter(|design| validate_design(&mut pattern_cache, &input.0, design))
        .count()
}

fn validate_design(
    cache: &mut HashMap<Vec<u8>, bool>,
    patterns: &[Vec<u8>],
    design: &[u8],
) -> bool {
    if let Some(cached) = cache.get(design) {
        return *cached;
    }
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            if remaining.is_empty() || validate_design(cache, patterns, remaining) {
                cache.insert(design.to_vec(), true);
                return true;
            }
        }
    }
    cache.insert(design.to_vec(), false);
    false
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &(Vec<Vec<u8>>, Vec<Vec<u8>>)) -> usize {
    let mut pattern_cache: HashMap<Vec<u8>, bool> = HashMap::new();
    let mut variant_cache: HashMap<Vec<u8>, usize> = HashMap::new();

    input
        .1
        .iter()
        .filter(|design| validate_design(&mut pattern_cache, &input.0, design))
        .map(|design| count_variants(&mut variant_cache, &input.0, design))
        .sum()
}

fn count_variants(
    cache: &mut HashMap<Vec<u8>, usize>,
    patterns: &[Vec<u8>],
    design: &[u8],
) -> usize {
    if let Some(cached) = cache.get(design) {
        return *cached;
    }
    let mut variants = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            if remaining.is_empty() {
                variants += 1;
            } else {
                variants += count_variants(cache, patterns, remaining);
            }
        }
    }
    cache.insert(design.to_vec(), variants);
    variants
}

#[cfg(test)]
mod tests {
    use crate::day19::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#,
        );
        assert_eq!(solve_part1(&input), 6);
        assert_eq!(solve_part2(&input), 16);
    }
}
