use std::fs;

fn part1() -> usize {
    let mut tally = 0;
    let file = fs::read_to_string("inputs/day1.txt").unwrap();
    let parsed: Vec<_> = file
        .lines()
        .map(|x| {
            let digits: Vec<_> = x.chars().filter(|x| x.is_ascii_digit()).collect();
            (x, digits)
        })
        .collect();
    for (_string, digits) in parsed {
        let firstlast: String = [digits.first().unwrap(), digits.last().unwrap()]
            .into_iter()
            .collect();
        let caliban = firstlast.parse::<usize>().unwrap();
        tally += caliban;
    }
    tally
}

fn line2digits(line: &str) -> Vec<&str> {
    let mut digits = Vec::<(usize, &str)>::new();
    for pattern in [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "eight", "five", "four", "nine", "one",
        "seven", "six", "three", "two",
    ] {
        let matches: Vec<(usize, &str)> = line.match_indices(pattern).collect();
        digits.extend(matches);
    }
    digits.sort_unstable_by(|(i1, _), (i2, _)| i1.cmp(i2));
    digits
        .iter()
        .map(|(_, d)| match d.len() {
            1 => d,
            _ => match *d {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => panic!("{:?} isn't a known digit or digit word", d),
            },
        })
        .collect()
}

fn digits2number(digits: Vec<&str>) -> usize {
    let firstlast: String = vec![digits.first(), digits.last()]
        .into_iter()
        .map(|x| *x.unwrap())
        .collect();
    firstlast.parse::<usize>().unwrap()
}

fn part2() -> usize {
    let file = fs::read_to_string("inputs/day1.txt").unwrap();
    let parsed: Vec<_> = file.lines().map(|x| (x, line2digits(x))).collect();
    assert_eq!(parsed.len(), 1000);
    let mut numbers = Vec::<usize>::new();
    for (_string, digits) in parsed {
        let number = digits2number(digits);
        numbers.push(number);
    }
    assert_eq!(numbers.len(), 1000);
    println!("{:?}", numbers);
    numbers.iter().sum()
}

pub fn go() {
    part1();
    println!("{:?}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line2number(line: &str) -> usize {
        digits2number(line2digits(line))
    }

    #[test]
    fn no_digits() {
        assert_eq!(line2digits("lmao"), Vec::<&str>::new());
    }

    #[test]
    #[should_panic]
    fn no_digits_full() {
        line2number("lmao");
    }

    #[test]
    fn overlapping_digit_words_count_twice() {
        assert_eq!(
            line2digits("3buttsoneightwolmao7"),
            vec!["3", "1", "8", "2", "7"]
        );
    }
}
