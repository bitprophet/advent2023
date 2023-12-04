use std::collections::HashSet;

use aoc_runner_derive::aoc;

fn get_scores(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':').nth(1).unwrap().split('|').map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse().unwrap())
                    .collect::<HashSet<usize>>()
            });
            // My kingdom for a combo of split_whitespace and split_once...
            let (winners, numbers) = (parts.next().unwrap(), parts.next().unwrap());
            match winners.intersection(&numbers).count() {
                0 => 0,
                n => 2usize.pow((n - 1).try_into().unwrap()),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn winning(input: &str) -> usize {
    get_scores(input).iter().sum()
}

#[aoc(day4, part2)]
fn piles(input: &str) -> usize {
    let table = get_scores(input);
    table.len() // wrong
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_solve() {
        let sample = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .trim();
        assert_eq!(winning(sample), 13);
        assert_eq!(piles(sample), 30);
    }
}
