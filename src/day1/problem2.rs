#[derive(Clone)]
struct Point {
    x: isize,
    y: isize,
}

struct Route {
    face_to: Direction,
    journey: Vec<Point>,
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

impl Route {
    fn make_move(&mut self, rotation: Rotation, distance: usize) -> Option<usize> {
        self.face_to.rotate(&rotation);
        let last_position = self.journey.last().unwrap().clone();
        for i in 1..(distance+1) as isize {
            let position = match self.face_to {
                Direction::North => Point { y: last_position.y + i, ..last_position},
                Direction::East  => Point { x: last_position.x + i, ..last_position},
                Direction::South => Point { y: last_position.y - i, ..last_position},
                Direction::West  => Point { x: last_position.x - i, ..last_position},
            };

            if let Some(_) = self.journey.iter().position(|ref p| p.x == position.x && p.y == position.y) {
                return Some((position.x.abs() + position.y.abs()) as usize);
            }

            self.journey.push(position);
        }
        None
    }
}

impl Default for Route {
    fn default() -> Self {
        Route {
            face_to: Direction::North,
            journey: vec![Point {
                x: 0,
                y: 0,
            }],
        }
    }
}

impl Direction {
    fn rotate(&mut self, m: &Rotation) {
        *self = match m {
            &Rotation::Left => {
                match *self {
                    Direction::North => Direction::West,
                    Direction::East  => Direction::North,
                    Direction::South => Direction::East,
                    Direction::West  => Direction::South,
                }
            },
            &Rotation::Right => {
                match *self {
                    Direction::North => Direction::East,
                    Direction::East  => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West  => Direction::North,
                }
            },
        };
    }
}

pub fn run(input: &str) -> Option<usize> {
    let mut route = Route::default();

    for m in input.split(", ") {
        let mut chars = m.chars();

        let rotation = match chars.next().unwrap() {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _ => unreachable!(),
        };

        let distance = chars.as_str().parse().unwrap();

        if let Some(distance) = route.make_move(rotation, distance) {
            return Some(distance);
        }
    }

    None
}

pub fn challenge() -> Option<usize> {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day1-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn parsed_sample() {
    let mut route = Route::default();

    // R8
    assert_eq!(route.make_move(Rotation::Right, 8), None);

    // R4
    assert_eq!(route.make_move(Rotation::Right, 4), None);

    // R4
    assert_eq!(route.make_move(Rotation::Right, 4), None);

    // R8
    assert_eq!(route.make_move(Rotation::Right, 8), Some(4));
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        assert_eq!(super::run("R8, R4, R4, R8"), Some(4));
    }
}