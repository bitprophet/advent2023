use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn schemattic(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_solve() {
        let sample = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        .trim();
        assert_eq!(schemattic(sample), 4361);
    }
}
