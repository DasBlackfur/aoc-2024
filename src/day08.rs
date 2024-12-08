use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u32 {
    //let mut dbg_input = input.to_vec();
    let mut antennas: Vec<(char, isize, isize)> = Vec::new();
    input.iter().enumerate().for_each(|x| {
        x.1.iter().enumerate().for_each(|y| {
            if y.1 != &'.' {
                antennas.push((*y.1, x.0 as isize, y.0 as isize));
            }
        })
    });

    let mut locations: Vec<(isize, isize)> = Vec::new();
    antennas.iter().for_each(|a| {
        antennas.iter().filter(|a1| a1.0 == a.0).for_each(|a1| {
            let x = a.1 - (a1.1);
            let y = a.2 - (a1.2);

            if (a.1 + x, a.2 + y) != (a1.1, a1.2) {
                locations.push((a.1 + x, a.2 + y));
            }
            if (a1.1 - x, a1.2 - y) != (a.1, a.2) {
                locations.push((a1.1 - x, a1.2 - y));
            }
        });
    });

    //locations.iter().for_each(|l| {
    //    if let Some(x) = dbg_input.get_mut(l.0 as usize) {
    //        if let Some(y) = x.get_mut(l.1 as usize) {
    //            *y = '#';
    //        }
    //    }
    //});

    //dbg_input
    //    .iter()
    //    .for_each(|line| println!("{}", line.iter().collect::<String>()));

    locations
        .iter()
        .filter(|l| {
            l.0 >= 0 && l.0 < input.len() as isize && l.1 >= 0 && l.1 < input[0].len() as isize
        })
        .unique()
        .count() as u32
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u32 {
    //let mut dbg_input = input.to_vec();
    let mut antennas: Vec<(char, isize, isize)> = Vec::new();
    input.iter().enumerate().for_each(|x| {
        x.1.iter().enumerate().for_each(|y| {
            if y.1 != &'.' {
                antennas.push((*y.1, x.0 as isize, y.0 as isize));
            }
        })
    });

    let mut locations: Vec<(isize, isize)> = Vec::new();
    antennas.iter().for_each(|a| {
        antennas.iter().filter(|a1| a1.0 == a.0).for_each(|a1| {
            for i in 0..100 {
                let x = (a.1 - (a1.1)) * i;
                let y = (a.2 - (a1.2)) * i;

                locations.push((a.1 + x, a.2 + y));
                locations.push((a1.1 - x, a1.2 - y));
            }
        });
    });

    //locations.iter().for_each(|l| {
    //    if let Some(x) = dbg_input.get_mut(l.0 as usize) {
    //        if let Some(y) = x.get_mut(l.1 as usize) {
    //            *y = '#';
    //        }
    //    }
    //});

    //dbg_input
    //    .iter()
    //    .for_each(|line| println!("{}", line.iter().collect::<String>()));

    locations
        .iter()
        .filter(|l| {
            l.0 >= 0 && l.0 < input.len() as isize && l.1 >= 0 && l.1 < input[0].len() as isize
        })
        .unique()
        .count() as u32
}

#[cfg(test)]
mod tests {
    use crate::day08::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(
            r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#,
        );
        assert_eq!(solve_part1(&input), 14);
        assert_eq!(solve_part2(&input), 34);
    }
}
