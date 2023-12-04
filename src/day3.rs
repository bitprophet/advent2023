use aoc_runner_derive::aoc;

// TODO: gotta be something more compact lol
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

// NOTE: doing things (y, x) style because brain goes row->col? meh

// NOTE: there are plentiful grid libs but so far trying to save time has
// mostly screwed me up, so let's try doing this regular for now

// NOTE: trying to essentially wrap a Vec seems to get hairy fast re: weird
// peculiarities of things like impl Index. Not worth it yet...
type Row = Vec<char>;
type Cell = char;

#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Debug)]
struct Number {
    start: Point,
    chars: Vec<char>,
}

#[derive(Debug)]
struct Schematic {
    rows: Vec<Row>,
}

impl Schematic {
    fn size(&self) -> usize {
        self.rows.len() * self.rows.iter().max_by_key(|x| x.len()).unwrap().len()
    }

    fn show(&self) -> String {
        let mut output = String::new();
        for row in &self.rows {
            for cell in row {
                output.push(*cell);
            }
            output.push('\n');
        }
        output
    }

    fn get(&self, y: usize, x: usize) -> Cell {
        self.rows[y][x]
    }

    fn number_is_label(&self, number: &Number) -> bool {
        // TODO: replaceme
        number.chars.is_empty()
    }

    fn sum_part_numbers(&self) -> usize {
        let mut sum = 0;
        // TODO: feels like this 'wants' to be an Option<Number> but I'm not
        // clear on how to modify the internal value w/o a lot of vexing
        // full-object recreation (eg option.replace(...))
        // So, start with a bogus number and set a flag to false.
        let mut cur = Number {
            start: Point { y: 0, x: 0 },
            chars: Vec::new(),
        };
        let mut active = false;
        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // Numberriffic
                if DIGITS.contains(cell) {
                    // Starting a new number.
                    if !active {
                        cur = Number {
                            start: Point { y, x },
                            chars: vec![*cell],
                        };
                        active = true;
                    // Continuing the current number.
                    } else {
                        cur.chars.push(*cell);
                    }
                // Not numberriffic: did we just finish a number?
                } else if active {
                    active = false;
                    // And was that number seemingly adjacent to any 'parts'?
                    if self.number_is_label(&cur) {
                        sum += cur
                            .chars
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                    }
                }
            }
        }
        sum
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        Self {
            rows: value
                .lines()
                .map(|line| Row::from(line.chars().collect::<Vec<_>>()))
                .collect(),
        }
    }
}

#[aoc(day3, part1)]
fn schemattic(input: &str) -> usize {
    let schematic = Schematic::from(input);
    dbg!(schematic.show());
    schematic.sum_part_numbers()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parsing() {
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
        let s = Schematic::from(sample);
        assert_eq!(s.rows.len(), 10);
        assert_eq!(s.rows[0].len(), 10);
        assert_eq!(s.rows[0][2], '7');
        assert_eq!(s.get(0, 2), '7');
    }

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
