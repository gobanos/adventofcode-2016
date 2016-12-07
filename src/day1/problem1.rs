struct Position {
    face_to: Direction,
    x: isize,
    y: isize,
}

enum Rotation {
    Left,
    Right,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Position {
    fn make_move(&mut self, rotation: Rotation, distance: usize) {
        self.face_to.rotate(&rotation);
        match self.face_to {
            Direction::North => self.y += distance as isize,
            Direction::East => self.x += distance as isize,
            Direction::South => self.y -= distance as isize,
            Direction::West => self.x -= distance as isize,
        }
    }

    fn distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            face_to: Direction::North,
            x: 0,
            y: 0,
        }
    }
}

impl Direction {
    fn rotate(&mut self, m: &Rotation) {
        *self = match *m {
            Rotation::Left => {
                match *self {
                    Direction::North => Direction::West,
                    Direction::East => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West => Direction::South,
                }
            }
            Rotation::Right => {
                match *self {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
        };
    }
}

pub fn run(input: &str) -> usize {
    let mut pos = Position::default();

    for m in input.split(", ") {
        let mut chars = m.chars();

        let rotation = match chars.next().unwrap() {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _ => unreachable!(),
        };

        let distance = chars.as_str().parse().unwrap();

        pos.make_move(rotation, distance);
    }

    pos.distance()
}

pub fn challenge() -> usize {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day1-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn parsed_sample1() {
    let mut pos = Position::default();

    // R2
    pos.make_move(Rotation::Right, 2);

    // L3
    pos.make_move(Rotation::Left, 3);

    assert_eq!(pos.distance(), 5);
}

#[test]
fn parsed_sample2() {
    let mut pos = Position::default();

    // R2
    pos.make_move(Rotation::Right, 2);

    // R2
    pos.make_move(Rotation::Right, 2);

    // R2
    pos.make_move(Rotation::Right, 2);

    assert_eq!(pos.distance(), 2);
}

#[test]
fn parsed_sample3() {
    let mut pos = Position::default();

    // R5
    pos.make_move(Rotation::Right, 5);

    // L5
    pos.make_move(Rotation::Left, 5);

    // R5
    pos.make_move(Rotation::Right, 5);

    // R3
    pos.make_move(Rotation::Right, 3);

    assert_eq!(pos.distance(), 12);
}

#[cfg(test)]
mod test {
    #[test]
    fn sample1() {
        assert_eq!(super::run("R2, L3"), 5);
    }

    #[test]
    fn sample2() {
        assert_eq!(super::run("R2, R2, R2"), 2);
    }

    #[test]
    fn sample3() {
        assert_eq!(super::run("R5, L5, R5, R3"), 12);
    }
}
