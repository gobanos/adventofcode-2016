struct Keypad {
    pad: [[char; 5]; 5],
    row: usize,
    col: usize,
}

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Keypad {
    fn current(&self) -> char {
        self.pad[self.row][self.col]
    }

    fn follow(&mut self, instructions: &[Direction]) -> char {
        for direction in instructions.iter() {
            let mut x = self.col as isize - 2;
            let mut y = self.row as isize - 2;

            match direction {
                &Direction::Up => {
                    y -= 1;
                    if x.abs() + y.abs() <= 2 {
                        self.row -= 1;
                    }
                },
                &Direction::Right => {
                    x += 1;
                    if x.abs() + y.abs() <= 2 {
                        self.col += 1;
                    }
                },
                &Direction::Left => {
                    x -= 1;
                    if x.abs() + y.abs() <= 2 {
                        self.col -= 1;
                    }
                },
                &Direction::Down => {
                    y += 1;
                    if x.abs() + y.abs() <= 2 {
                        self.row += 1;
                    }
                },
            }
        }

        self.current()
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Keypad {
            pad: [
                [' ', ' ', '1', ' ', ' '],
                [' ', '2', '3', '4', ' '],
                ['5', '6', '7', '8', '9'],
                [' ', 'A', 'B', 'C', ' '],
                [' ', ' ', 'D', ' ', ' '],
            ],
            row: 2,
            col: 0,
        }
    }
}

pub fn run(input: &str) -> String {
    use self::Direction::*;

    let mut keypad = Keypad::default();
    let mut code = String::default();

    for line in input.lines() {
        let instructions = line.trim().chars().map(|c| {
            match c {
                'U' => Up,
                'R' => Right,
                'L' => Left,
                'D' => Down,
                _ =>  unreachable!(),
            }
        });

        code.push(keypad.follow(&instructions.collect::<Vec<_>>()));
    }

    code
}

pub fn challenge() -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day2-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn parsed_sample() {
    use self::Direction::*;

    let mut keypad = Keypad::default();

    // ULL
    assert_eq!(keypad.follow(&[Up, Left, Left]), '5');

    // RRDDD
    assert_eq!(keypad.follow(&[Right, Right, Down, Down, Down]), 'D');

    // LURDL
    assert_eq!(keypad.follow(&[Left, Up, Right, Down, Left]), 'B');

    // UUUUD
    assert_eq!(keypad.follow(&[Up, Up, Up, Up, Down]), '3');
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(super::run(input), "5DB3");
    }
}
