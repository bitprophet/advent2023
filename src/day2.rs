use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, PartialOrd)]
struct Pull {
    red: usize,
    green: usize,
    blue: usize,
}

impl Pull {
    fn fits_inside(&self, bag: &Self) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
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
                // NOTE: many of the split() calls here used to include the
                // trailing whitespace, but was tightened up in case of sneaky
                // not-quite-mis-formatting or whatever. (not that I found any)
                .split(';')
                .filter(|chunk| !chunk.is_empty())
                .map(|chunk| {
                    // TODO: am I missing some metaprogramming feature that
                    // would let me go from variables-holding-field-names to
                    // the struct directly?
                    let color_counts: Vec<_> = chunk
                        .trim()
                        .split(',')
                        .filter(|x| !x.is_empty())
                        .map(|subchunk| {
                            let (count, cname) = subchunk.trim().split_once(' ').unwrap();
                            (cname, count.parse().unwrap())
                        })
                        .collect();
                    for color in ["red", "green", "blue"] {
                        // Check: no color was given twice within a
                        // single 'pull' statement.
                        assert!(
                            color_counts
                                .iter()
                                .filter(|(cname, _)| cname == &color)
                                .collect::<Vec<_>>()
                                .len()
                                <= 1
                        );
                    }
                    let mut map = HashMap::<&str, usize>::from_iter(color_counts);
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
    let mut tally = 0;
    let mut possible: Vec<Game> = vec![];
    let mut impossible: Vec<Game> = vec![];
    for game in games {
        // Checks: totally empty games or pulls (not that this SHOULD be fatal
        // but)
        if game
            .pulls
            .iter()
            .any(|x| x.red == 0 && x.green == 0 && x.blue == 0)
            || game.pulls.is_empty()
        {
            println!("{:?}", game);
        }
        if game.pulls.iter().all(|x| x.fits_inside(&bag)) {
            tally += game.id;
            possible.push(game);
        } else {
            impossible.push(game);
        }
    }
    println!(
        "possible games: {:?}, impossible: {:?}",
        possible.len(),
        impossible.len()
    );
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
        // NOTE: extra semicolons and commas!
        assert_eq!(
            generate_games("Game 27: 1 red,;; 2 green; 3 blue,;"),
            vec![Game {
                id: 27,
                pulls: vec![
                    Pull {
                        red: 1,
                        green: 0,
                        blue: 0
                    },
                    Pull {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                    Pull {
                        red: 0,
                        green: 0,
                        blue: 3
                    }
                ]
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
    fn fits_inside() {
        let bag = Pull {
            red: 1,
            green: 1,
            blue: 1,
        };
        assert!(Pull {
            red: 1,
            green: 1,
            blue: 1
        }
        .fits_inside(&bag));
        assert!(Pull {
            red: 0,
            green: 0,
            blue: 0
        }
        .fits_inside(&bag));
        assert!(!Pull {
            red: 1,
            green: 1,
            blue: 2
        }
        .fits_inside(&bag));
        assert!(!Pull {
            red: 11,
            green: 111,
            blue: 0
        }
        .fits_inside(&bag));
    }
}
