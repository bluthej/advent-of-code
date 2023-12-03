use std::fs;

use anyhow::{anyhow, Result};
use itertools::Itertools;

const PATH: &str = "./input/day03.txt";

fn main() -> Result<()> {
    let (part1, part2) = solve(PATH)?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

pub fn solve(path: &str) -> Result<(usize, usize)> {
    let contents = fs::read_to_string(path)?;

    let part1 = contents
        .lines()
        .flat_map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            [comp1, comp2].into_iter()
        })
        .compute_priority(2)?;

    let part2 = contents.lines().compute_priority(3)?;

    Ok((part1, part2))
}

trait PriorityComputer {
    fn compute_priority(self, n: usize) -> Result<usize>;
}

impl<'a, I: Iterator<Item = &'a str>> PriorityComputer for I {
    fn compute_priority(self, n: usize) -> Result<usize> {
        let chars = ('a'..='z').chain('A'..='Z').collect::<Vec<char>>();
        self.chunks(n)
            .into_iter()
            .map(|chunk| {
                chunk
                    .fold(chars.clone(), |mut common, line| {
                        common.retain(|&c| line.contains(c));
                        common
                    })
                    .first()
                    .and_then(|&common| chars.iter().position(|&c| c == common))
                    .map(|val| val + 1)
                    .ok_or(anyhow!("Common character not found"))
            })
            .sum::<Result<usize>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let (part1, part2) = solve(PATH)?;
        assert_eq!(part1, 7691);
        assert_eq!(part2, 2508);
        Ok(())
    }
}
