use std::collections::BTreeSet;

pub fn solve_a(input: &str) -> u64 {
    input
        .lines()
        .enumerate()
        .filter(|(index, _)| index & 1 == 0)
        .map(|(_, line)| line)
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(index, _)| index)
                .collect::<BTreeSet<_>>()
        })
        .map(|x| (0, x))
        .reduce(|(nr, acc), (_, e)| {
            let (collisions, misses): (BTreeSet<_>, BTreeSet<_>) =
                acc.into_iter().partition(|n| e.contains(n));
            (
                nr + collisions.len(),
                collisions
                    .into_iter()
                    .flat_map(|n| [n - 1, n + 1])
                    .chain(misses)
                    .collect(),
            )
        })
        .unwrap()
        .0
        .try_into()
        .unwrap()
}

pub fn solve_b(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    const INPUT: &'static str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_a() {
        assert_eq!(super::solve_a(INPUT), 21);
    }

    #[test]
    fn test_b() {
        assert_eq!(super::solve_b(INPUT), 40);
    }
}
