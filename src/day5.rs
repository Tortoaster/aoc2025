use std::str::FromStr;

use anyhow::{Context, Error, anyhow};

pub fn solve_a(input: &str) -> u64 {
    let (ranges, ids) = parse(input);
    ids.into_iter()
        .filter(|&id| ranges.iter().any(|range| range.contains(id)))
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_b(input: &str) -> u64 {
    let (mut ranges, _) = parse(input);
    ranges.sort_by(|a, b| a.from.cmp(&b.from));
    ranges
        .into_iter()
        .fold(Vec::<Range>::new(), |mut acc, e| {
            let mut joined = false;
            for range in acc.iter_mut() {
                if range.overlaps(&e) {
                    range.join_unchecked(&e);
                    joined = true;
                }
            }
            if !joined {
                acc.push(e);
            }
            acc
        })
        .into_iter()
        .map(|range| range.len())
        .sum()
}

fn parse(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(Range::from_str)
        .map(Result::unwrap)
        .collect();
    let ids = ids.lines().map(|id| id.parse().unwrap()).collect();
    (ranges, ids)
}

struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn len(self) -> u64 {
        self.to - self.from + 1
    }

    fn contains(&self, n: u64) -> bool {
        n >= self.from && n <= self.to
    }

    fn overlaps(&self, other: &Self) -> bool {
        if self.from <= other.from && self.to >= other.to
            || other.from <= self.from && other.to >= self.to
        {
            true
        } else {
            self.from.max(other.from) <= self.to.min(other.to)
        }
    }

    fn join_unchecked(&mut self, other: &Self) {
        self.from = self.from.min(other.from);
        self.to = self.to.max(other.to);
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').ok_or(anyhow!("No hyphen in {s}"))?;
        let from = from
            .parse()
            .with_context(|| format!("Failed to parse range start of {s}"))?;
        let to = to
            .parse()
            .with_context(|| format!("Failed to parse range end of {s}"))?;
        Ok(Range { from, to })
    }
}
