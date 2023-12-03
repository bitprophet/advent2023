use aoc_runner_derive::aoc;

// NOTE: there are plentiful grid libs but so far trying to save time has
// mostly screwed me up, so let's try doing this regular for now

// NOTE: trying to essentially wrap a Vec seems to get hairy fast re: weird
// peculiarities of things like impl Index. Not worth it yet...
type Row = Vec<char>;

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
    println!("{}", schematic.show());
    schematic.size()
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
        assert_eq!(s.rows[0][0], '4');
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
