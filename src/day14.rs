use aoc_runner_derive::{aoc, aoc_generator};
use image::{ImageBuffer, Luma};
use regex::Regex;

#[derive(Clone, Copy, Debug)]
pub struct Robot {
    pos_x: isize,
    pos_y: isize,
    vel_x: isize,
    vel_y: isize,
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Robot> {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=([-\d]+),([-\d]+)").unwrap();
    robot_regex
        .captures_iter(input)
        .map(|robot_capture| {
            let robot = robot_capture.extract::<4>().1;
            Robot {
                pos_x: robot[0].parse().unwrap(),
                pos_y: robot[1].parse().unwrap(),
                vel_x: robot[2].parse().unwrap(),
                vel_y: robot[3].parse().unwrap(),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[Robot]) -> usize {
    let positions: Vec<(isize, isize)> = input
        .iter()
        .map(|robot| {
            let mut robot_simulated = *robot;

            robot_simulated.pos_x += robot_simulated.vel_x * 100;
            robot_simulated.pos_x %= 101;
            if robot_simulated.pos_x < 0 {
                robot_simulated.pos_x += 101;
            }

            robot_simulated.pos_y += robot_simulated.vel_y * 100;
            robot_simulated.pos_y %= 103;
            if robot_simulated.pos_y < 0 {
                robot_simulated.pos_y += 103;
            }

            (robot_simulated.pos_x, robot_simulated.pos_y)
        })
        .collect();

    let up_left = positions
        .iter()
        .filter(|pos| pos.0 < 50 && pos.1 < 51)
        .count();
    let up_right = positions
        .iter()
        .filter(|pos| pos.0 > 50 && pos.1 < 51)
        .count();
    let down_left = positions
        .iter()
        .filter(|pos| pos.0 < 50 && pos.1 > 51)
        .count();
    let down_right = positions
        .iter()
        .filter(|pos| pos.0 > 50 && pos.1 > 51)
        .count();

    up_left * up_right * down_left * down_right
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[Robot]) -> usize {
    let mut steps = 1;
    let mut image = ImageBuffer::<Luma<u8>, Vec<u8>>::new(101, 103);
    loop {
        let positions: Vec<(isize, isize)> = input
            .iter()
            .map(|robot| {
                let mut robot_simulated = *robot;

                robot_simulated.pos_x += robot_simulated.vel_x * steps;
                robot_simulated.pos_x %= 101;
                if robot_simulated.pos_x < 0 {
                    robot_simulated.pos_x += 101;
                }

                robot_simulated.pos_y += robot_simulated.vel_y * steps;
                robot_simulated.pos_y %= 103;
                if robot_simulated.pos_y < 0 {
                    robot_simulated.pos_y += 103;
                }

                (robot_simulated.pos_x, robot_simulated.pos_y)
            })
            .collect();

        println!("{}", steps);

        image.fill(255);
        for pos in positions {
            image.put_pixel(pos.0 as u32, pos.1 as u32, Luma([0]));
        }

        image.save(format!("{steps}.jpg")).unwrap();
        steps += 1;
    }
}
