use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    File(usize),
    Blank,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Block> {
    let mut disk = Vec::new();
    let mut id = 0;
    let mut block = true;
    let block_ref = &mut block;
    input.chars().for_each(|val| {
        if *block_ref {
            disk.append(&mut vec![
                Block::File(id);
                val.to_digit(10).unwrap() as usize
            ]);
            *block_ref = false;
            id += 1;
        } else {
            disk.append(&mut vec![Block::Blank; val.to_digit(10).unwrap() as usize]);
            *block_ref = true;
        }
    });
    disk
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Block]) -> usize {
    //println!("{:?}", input);
    let mut result = input.to_vec();
    let mut exit = false;
    let exit_ref = &mut exit;

    let exit_pos = input
        .iter()
        .filter(|block| matches!(block, Block::File(_)))
        .count();

    input.iter().rev().for_each(|block| {
        if !*exit_ref {
            match result
                .iter()
                .position(|search_block| matches!(search_block, Block::Blank))
            {
                Some(pos) => {
                    result[pos] = *block;
                }
                None => *exit_ref = true,
            }
        }
    });

    result = result[0..exit_pos].to_vec();

    //println!("{:?}", result);

    result
        .iter()
        .enumerate()
        .map(|(pos, block)| match block {
            Block::File(value) => pos * value,
            Block::Blank => unreachable!(),
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Block]) -> usize {
    let mut chunked_input: Vec<(usize, Block)> = Vec::new();
    input
        .iter()
        .for_each(|block| match chunked_input.last_mut() {
            Some(file) => {
                if *block == file.1 {
                    file.0 += 1;
                } else {
                    chunked_input.push((1, *block));
                }
            }
            None => chunked_input.push((1, *block)),
        });

    let mut chunked_result = chunked_input.clone();

    let mut largest = chunked_input
        .iter()
        .filter(|chunk| matches!(chunk.1, Block::File(_)))
        .map(|chunk| match chunk.1 {
            Block::File(value) => value,
            Block::Blank => unreachable!(),
        })
        .max()
        .unwrap()
        + 1;

    while largest != 0 {
        //println!("{:?}", chunked_result);
        largest -= 1;
        let (pos, current_chunk) = chunked_input
            .iter()
            .enumerate()
            .find(|(_pos, chunk)| match chunk.1 {
                Block::File(num) => num == largest,
                Block::Blank => false,
            })
            .unwrap();

        if let Some((replace_pos, replace_chunk)) =
            chunked_input
                .iter()
                .enumerate()
                .find(|(_pos, chunk)| match chunk.1 {
                    Block::File(_) => false,
                    Block::Blank => chunk.0 >= current_chunk.0,
                })
        {
            if replace_pos < pos {
                chunked_result[pos] = (current_chunk.0, Block::Blank);
                chunked_result[replace_pos] = *current_chunk;
                let remaining = replace_chunk.0 - current_chunk.0;
                if remaining > 0 {
                    chunked_result.insert(replace_pos + 1, (remaining, Block::Blank));
                }
            }
        }
        chunked_input = chunked_result.clone();
    }

    let mut unchunked = Vec::new();

    chunked_result.iter().for_each(|chunk| {
        unchunked.append(&mut vec![chunk.1; chunk.0]);
    });

    unchunked
        .iter()
        .enumerate()
        .map(|(pos, block)| match block {
            Block::File(value) => pos * value,
            Block::Blank => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day09::{input_generator, solve_part1, solve_part2};

    #[test]
    fn task_sample() {
        let input = input_generator(r#"2333133121414131402"#);
        assert_eq!(solve_part1(&input), 1928);
        assert_eq!(solve_part2(&input), 2858);
    }
}
