use std::str::FromStr;

use anyhow::{Error, anyhow};

pub fn solve_a(input: &str) -> u64 {
    parse(input).map(Expr::eval).sum()
}

pub fn solve_b(input: &str) -> u64 {
    0
}

fn parse(input: &str) -> impl Iterator<Item = Expr> {
    let matrix: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let transposed: Vec<Vec<_>> = (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
        .collect();
    transposed.into_iter().map(|parts| Expr::from_parts(&parts))
}

struct Expr {
    nums: Vec<u64>,
    op: Op,
}

impl Expr {
    fn from_parts(parts: &[&str]) -> Self {
        let nums = parts[0..parts.len() - 1]
            .iter()
            .map(|n| n.parse().unwrap())
            .collect();
        let op = parts.last().unwrap().parse().unwrap();
        Expr { nums, op }
    }

    fn eval(self) -> u64 {
        match self.op {
            Op::Add => self.nums.into_iter().sum(),
            Op::Mul => self.nums.into_iter().product(),
        }
    }
}

enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(anyhow!("{s} is not a valid operation")),
        }
    }
}
