use anyhow::{Result, anyhow};
use std::{
    iter::{self, RepeatN},
    str::FromStr,
};

pub fn solve_a(input: &str) -> u64 {
    input
        .lines()
        .map(Rotation::from_str)
        .map(Result::unwrap)
        .scan(50, rotate)
        .filter(|&x| x == 0)
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_b(input: &str) -> u64 {
    input
        .lines()
        .map(Rotation::from_str)
        .map(Result::unwrap)
        .flatten()
        .scan(50, rotate)
        .filter(|&x| x == 0)
        .count()
        .try_into()
        .unwrap()
}

fn rotate(number: &mut i64, rotation: Rotation) -> Option<i64> {
    *number = (*number + rotation.0).rem_euclid(100);
    Some(*number)
}

#[derive(Clone)]
struct Rotation(i64);

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (direction, distance) = s.split_at(1);
        let distance = distance.parse()?;
        match direction {
            "R" => Ok(Rotation(distance)),
            "L" => Ok(Rotation(-distance)),
            c => Err(anyhow!("{c} is not a valid direction")),
        }
    }
}

impl IntoIterator for Rotation {
    type Item = Rotation;

    type IntoIter = RepeatN<Rotation>;

    fn into_iter(self) -> Self::IntoIter {
        iter::repeat_n(Rotation(self.0.signum()), self.0.abs().try_into().unwrap())
    }
}
