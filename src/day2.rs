use std::{collections::BTreeSet, str::FromStr};

use anyhow::{Context, Error, Result, anyhow};

pub fn solve_a(input: &str) -> u64 {
    parse_input(input)
        .unwrap()
        .iter()
        .flat_map(Range::invalid)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    parse_input(input)
        .unwrap()
        .iter()
        .flat_map(Range::also_invalid)
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Range>> {
    input.trim().split(',').map(Range::from_str).collect()
}

struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn invalid(&self) -> impl Iterator<Item = u64> {
        self.invalid_n(2)
    }

    fn also_invalid(&self) -> BTreeSet<u64> {
        [2, 3, 5, 7]
            .iter()
            .flat_map(|&n| self.invalid_n(n))
            .collect()
    }

    fn invalid_n(&self, n: u32) -> impl Iterator<Item = u64> {
        let from = first_nth(self.from, n);
        let to = first_nth(self.to, n) + 1;
        (from..to)
            .map(move |x| concat_n(x, n))
            .skip_while(|&x| !self.contains(x))
            .take_while(|&x| self.contains(x))
    }

    fn contains(&self, n: u64) -> bool {
        self.from <= n && self.to >= n
    }
}

fn first_nth(x: u64, n: u32) -> u64 {
    let log = x.ilog10();
    if (log + 1) % n == 0 {
        x / 10u64.pow((log + 1) - (log + 1) / n)
    } else {
        10u64.pow(log / n)
    }
}

fn concat_n(x: u64, n: u32) -> u64 {
    let mut acc = 0u64;
    let mul = 10u64.pow(x.ilog10() + 1);
    for _ in 0..n {
        acc = acc * mul + x;
    }
    acc
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or(anyhow!("No hyphen in {s}"))?;
        let from = from
            .parse()
            .with_context(|| format!("Failed to parse start of range {s}"))?;
        let to = to
            .parse()
            .with_context(|| format!("Failed to parse end of range {s}"))?;

        Ok(Range { from, to })
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_b() {
        assert_eq!(super::solve_b(INPUT), 4174379265)
    }
}
