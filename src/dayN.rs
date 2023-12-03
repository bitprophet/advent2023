use aoc_runner_derive::aoc;

#[aoc(dayN, part1)]
fn cutename(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_solve() {
        let sample = "the sample";
        assert_eq!(cutename(sample), 1);
    }
}
