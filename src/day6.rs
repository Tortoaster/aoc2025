use std::{iter, str::FromStr};

use anyhow::{Error, anyhow};
use itertools::Itertools;

pub fn solve_a(input: &str) -> u64 {
    let matrix: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    transpose(matrix)
        .into_iter()
        .map(|parts| Expr::from_parts(&parts))
        .map(Expr::eval)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let ops_line = input.lines().last().unwrap();
    let matrix = input
        .lines()
        .take(4)
        .chain(iter::once(
            &*iter::repeat_n(' ', ops_line.len()).collect::<String>(),
        ))
        .chain(iter::once(ops_line))
        .map(|line| line.chars().collect())
        .collect();

    transpose(matrix)
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .batching(|it| {
            let mut op = None;
            let mut nums = Vec::new();
            while let Some((num, o)) = batching_line(it) {
                nums.push(num);
                op = op.or(o);
            }
            Some(Expr { nums, op: op? })
        })
        .map(Expr::eval)
        .sum()
}

fn transpose<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    (0..cols)
        .map(|col| (0..rows).map(|row| matrix[row][col]).collect())
        .collect()
}

fn batching_line(it: &mut impl Iterator<Item = String>) -> Option<(u64, Option<Op>)> {
    let line = it.next()?;
    let mut parts = line.split_whitespace();
    let num = parts.next()?.parse().unwrap();
    let op = parts.next().map(Op::from_str).transpose().unwrap();
    Some((num, op))
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
