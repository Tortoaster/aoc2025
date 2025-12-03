use std::str::FromStr;

use anyhow::{Context, Error, Result, anyhow};

pub fn solve_a(input: &str) -> u64 {
    parse_input(input).unwrap().iter().flat_map(Range::invalid).sum()
}

pub fn solve_b(input: &str) -> u64 {
    0
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
        let from = first_half(self.from);
        let to = first_half(self.to) + 1;
        (from..to).map(double).skip_while(|&n| !self.contains(n)).take_while(|&n| self.contains(n))
    }

    fn contains(&self, n: u64) -> bool {
        self.from <= n && self.to > n
    }
}

fn first_half(n: u64) -> u64 {
    let log = n.ilog10();
    if log & 1 == 0 {
        10u64.pow(log / 2)
    } else {
        n / 10u64.pow((log + 1) / 2)
    }
}

fn double(n: u64) -> u64 {
    10u64.pow(n.ilog10() + 1) * n + n
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or(anyhow!("No hyphen in {s}"))?;
        let from = from.parse().with_context(|| format!("Failed to parse start of range {s}"))?;
        let to = to.parse().with_context(|| format!("Failed to parse end of range {s}"))?;

        Ok(Range { from, to })
    }
}
