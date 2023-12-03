use std::fs;

use anyhow::{Error, Result};

static PATH: &str = "./input/day01.txt";

fn main() -> Result<()> {
    let (part1, part2) = solve(PATH)?;
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

pub fn solve(path: &str) -> Result<(u64, u64)> {
    let mut cals = fs::read_to_string(path)?
        .split("\n\n")
        .map(|chunk| {
            chunk.lines().try_fold(0u64, |tot_cal, line| {
                line.parse()
                    .map_err(From::from)
                    .and_then(|cal| tot_cal.checked_add(cal).ok_or(Error::msg("Overflow")))
            })
        })
        .collect::<Result<Vec<_>>>()?;

    cals.sort_by(|a, b| b.cmp(a));

    let part1 = cals.iter().take(1).sum::<u64>();
    let part2 = cals.iter().take(3).sum::<u64>();

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let (part1, part2) = solve(PATH)?;
        assert_eq!(part1, 71124);
        assert_eq!(part2, 204639);
        Ok(())
    }
}
