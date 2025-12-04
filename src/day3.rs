pub fn solve_a(input: &str) -> u64 {
    parse(input).map(|bank| solve(&bank, 1)).sum()
}

pub fn solve_b(input: &str) -> u64 {
    parse(input).map(|bank| solve(&bank, 11)).sum()
}

fn parse(input: &str) -> impl Iterator<Item = Vec<u64>> {
    input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect()
    })
}

fn solve(bank: &[u64], left: usize) -> u64 {
    let max = bank[..bank.len() - left].iter().max().unwrap();
    let index = bank.iter().position(|&x| x == *max).unwrap();
    if left == 0 {
        *max
    } else {
        10u64.pow(left as u32) * max + solve(&bank[index + 1..], left - 1)
    }
}
