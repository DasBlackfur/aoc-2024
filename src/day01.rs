use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut list1, mut list2) = (Vec::<u32>::new(), Vec::<u32>::new());
    input.split("\n").for_each(|line| {
        let part1 = &line[..5];
        let part2 = &line[8..];
        list1.push(part1.parse().unwrap());
        list2.push(part2.parse().unwrap());
    });
    (list1, list2)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list1 = input.0.clone();
    let mut list2 = input.1.clone();

    list1.sort();
    list2.sort();

    list1.iter().zip(list2.iter()).map(|items| {
        items.0.abs_diff(*items.1)
    }).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let list1 = input.0.clone();
    let list2 = input.1.clone();

    list1.iter().map(|item1| {
        item1 * list2.iter().filter(|item2| item1 == *item2).count() as u32
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day01::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator("00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003");
        assert_eq!(solve_part1(&input), 11);
        assert_eq!(solve_part2(&input), 31);
    }
}