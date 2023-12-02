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

fn part2() -> usize {
    let mut tally = 0;
    let file = fs::read_to_string("inputs/day1.txt").unwrap();
    let pattern =
        regex::Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let parsed: Vec<_> = file
        .lines()
        .map(|x| {
            let matches: Vec<_> = pattern
                .find_iter(x)
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
                .collect();
            (x, matches)
        })
        .collect();
    for (string, digits) in parsed {
        let firstlast: String = vec![digits.first(), digits.last()]
            .into_iter()
            .map(|x| *x.unwrap())
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

pub fn go() {
    part1();
    //println!("Part 1 answer: {:?}", part1());
    println!("Part 2 answer: {:?}", part2());
}
