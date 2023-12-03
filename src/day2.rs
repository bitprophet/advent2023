use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, PartialOrd)]
struct Pull {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    pulls: Vec<Pull>,
}

fn generate_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once(": ").unwrap();
            let (_, id) = name.split_once(' ').unwrap();
            let pulls: Vec<Pull> = rest
                .split("; ")
                .map(|chunk| {
                    // TODO: am I missing some metaprogramming feature that
                    // would let me go from variables-holding-field-names to
                    // the struct directly?
                    let mut map =
                        HashMap::<&str, usize>::from_iter(chunk.split(", ").map(|subchunk| {
                            let (count, cname) = subchunk.split_once(' ').unwrap();
                            (cname, count.parse().unwrap())
                        }));
                    Pull {
                        red: map.remove("red").unwrap_or(0),
                        green: map.remove("green").unwrap_or(0),
                        blue: map.remove("blue").unwrap_or(0),
                    }
                })
                .collect();
            Game {
                id: id.parse().unwrap(),
                pulls,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn bored_elf(input: &str) -> usize {
    // You can think of "what is in the bag" as an extremely large meta-pull...
    let bag = Pull {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = generate_games(input);
    println!("{:?}", games);
    let mut tally = 0;
    for game in games {
        if game.pulls.iter().all(|x| x <= &bag) {
            tally += game.id;
        }
    }
    tally
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_generation() {
        assert_eq!(
            generate_games("Game 27: 1 red, 2 green, 3 blue"),
            vec![Game {
                id: 27,
                pulls: vec![Pull {
                    red: 1,
                    green: 2,
                    blue: 3
                }]
            }]
        );
        assert_eq!(
            generate_games("Game 27: 1 red"),
            vec![Game {
                id: 27,
                pulls: vec![Pull {
                    red: 1,
                    green: 0,
                    blue: 0
                }]
            }]
        );
    }

    #[test]
    fn description_sample() {
        assert_eq!(
            bored_elf(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        )
    }

    #[test]
    fn pull_comparisons() {
        assert_eq!(
            Pull {
                red: 1,
                green: 2,
                blue: 3
            },
            Pull {
                red: 1,
                green: 2,
                blue: 3
            }
        );
        assert!(
            Pull {
                red: 1,
                green: 2,
                blue: 3
            } > Pull {
                red: 1,
                green: 2,
                blue: 0
            }
        );
        assert!(
            Pull {
                red: 0,
                green: 0,
                blue: 1
            } > Pull {
                red: 0,
                green: 0,
                blue: 0
            }
        );
        assert!(
            Pull {
                red: 0,
                green: 1,
                blue: 2
            } < Pull {
                red: 0,
                green: 1,
                blue: 3
            }
        )
    }
}
