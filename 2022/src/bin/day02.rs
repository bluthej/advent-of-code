use std::{fs, str::FromStr};

use anyhow::{anyhow, Error, Result};

static PATH: &str = "./input/day02.txt";

struct Round(usize, usize);

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(' ')
            .ok_or(anyhow!("Wrong input. Got: {}", s))?;
        let first = "ABC"
            .find(first)
            .ok_or(anyhow!("Did not reckognize letter {}", first))?;
        let second = "XYZ"
            .find(second)
            .ok_or(anyhow!("Did not reckognize letter {}", second))?;

        let my_score_1 = 1 + second;
        let outcome_1 = (my_score_1 + 3 - first) % 3 * 3;
        let part1 = my_score_1 + outcome_1;

        let outcome_2 = second * 3;
        let my_score_2 = ((first + 3 + second - 1) % 3) + 1;
        let part2 = my_score_2 + outcome_2;
        Ok(Round(part1, part2))
    }
}

fn main() -> Result<()> {
    let (part1, part2) = solve(PATH)?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

pub fn solve(path: &str) -> Result<(usize, usize)> {
    fs::read_to_string(path)?
        .lines()
        .map(|line| {
            line.parse::<Round>()
                .map(|Round(part1, part2)| (part1, part2))
        })
        .try_fold((0usize, 0usize), |(tot1, tot2), part| {
            part.map(|(part1, part2)| (tot1 + part1, tot2 + part2))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let (part1, part2) = solve(PATH)?;
        assert_eq!(part1, 14827);
        assert_eq!(part2, 13889);
        Ok(())
    }
}
