use std::fs;

use regex;

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
    for (string, digits) in parsed {
        let firstlast: String = [digits.first().unwrap(), digits.last().unwrap()]
            .into_iter()
            .collect();
        let caliban = firstlast.parse::<usize>().unwrap();
        println!(
            "{:?} => {:?} => {:?} => {:?}",
            string, digits, firstlast, caliban
        );
        tally += caliban;
    }
    tally
}

fn line_to_digits(line: &str) -> Vec<&str> {
    let pattern = regex::Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    pattern
        .find_iter(line)
        .map(|y| {
            let digit = y.as_str();
            match digit.len() {
                1 => digit,
                _ => match digit {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => panic!("wtf? {:?} shouldn't have matched the regex!", digit),
                },
            }
        })
        .collect()
}

fn digits_to_number(digits: Vec<&str>) -> usize {
    let firstlast: String = vec![digits.first(), digits.last()]
        .into_iter()
        .map(|x| *x.unwrap())
        .collect();
    firstlast.parse::<usize>().unwrap()
}

fn part2() -> usize {
    let mut tally = 0;
    let file = fs::read_to_string("inputs/day1.txt").unwrap();
    let parsed: Vec<_> = file.lines().map(|x| (x, line_to_digits(x))).collect();
    for (string, digits) in parsed {
        print!("{:?} => {:?}", string, digits);
        let number = digits_to_number(digits);
        println!(" => {:?}", number);
        tally += number;
    }
    tally
}

pub fn go() {
    part1();
    //println!("Part 1 answer: {:?}", part1());
    println!("Part 2 answer: {:?}", part2());
}
