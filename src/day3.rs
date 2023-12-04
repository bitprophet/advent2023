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
    fn _show(&self) -> String {
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

    fn is_part(&self, point: &Point) -> bool {
        let cell = self.get(point.y, point.x);
        cell != '.' && !DIGITS.contains(&cell)
    }

    fn number_is_label(&self, num: &Number) -> bool {
        let start = &num.start;
        // Not strictly necessary but feels clean, might help with part 2?
        let mut adjacent_parts: Vec<Point> = Vec::new();
        // bounds
        let right = start.x + num.chars.len();
        let bottom = start.y + 1;
        // Look above the number (including corners).
        // (but not on 1st row...)
        if start.y > 0 {
            let top = start.y - 1;
            let row = &self.rows[top];
            let scan_start = if start.x >= 1 { start.x - 1 } else { start.x };
            let scan_end = if right < row.len() { right } else { right - 1 };
            for i in scan_start..=scan_end {
                let point = Point { y: top, x: i };
                if self.is_part(&point) {
                    adjacent_parts.push(point);
                }
            }
        }
        // Look left of the number.
        if start.x > 0 {
            let point = Point {
                y: start.y,
                x: start.x - 1,
            };
            if self.is_part(&point) {
                adjacent_parts.push(point);
            }
        }
        // Look right of the number.
        if right < self.rows[start.y].len() {
            let point = Point {
                y: start.y,
                x: right,
            };
            if self.is_part(&point) {
                adjacent_parts.push(point);
            }
        }
        // Look below the number (including corners).
        // (but not on last row...)
        if bottom < self.rows.len() {
            let row = &self.rows[bottom];
            let scan_start = if start.x >= 1 { start.x - 1 } else { start.x };
            let scan_end = if right < row.len() { right } else { right - 1 };
            for i in scan_start..=scan_end {
                let point = Point { y: bottom, x: i };
                if self.is_part(&point) {
                    adjacent_parts.push(point);
                }
            }
        }
        !adjacent_parts.is_empty()
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
    schematic.sum_part_numbers()
}

/*
#[aoc(day3, part2)]
fn ratioed(input: &str) -> usize {
    let schematic = Schematic::from(input);
    input.len()
}
*/

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
    fn scanning() {
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
        assert!(s.number_is_label(&Number {
            start: Point { y: 0, x: 0 },
            chars: vec!['4', '6', '7']
        }));
        assert!(!s.number_is_label(&Number {
            start: Point { y: 0, x: 5 },
            chars: vec!['1', '1', '4']
        }));
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
        //        assert_eq!(ratioed(sample), 536576);
    }
}
