use regex::Regex;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum Pixel {
    On,
    Off,
}

struct Screen {
    pixels: [[Pixel; 50]; 6],
}

impl Screen {
    fn rect(&mut self, dim: &Dimension) {
        assert!(dim.width <= 50);
        assert!(dim.height <= 6);

        for mut row in self.pixels[0..dim.height].iter_mut() {
            for mut pixel in row[0..dim.width].iter_mut() {
                *pixel = Pixel::On;
            }
        }
    }

    fn rotate_row(&mut self, rot: &Rotation) {
        assert!(rot.index < 6);
        assert!(rot.offset <= 50);

        let tmp = self.pixels[rot.index].iter().map(|p| *p).collect::<Vec<_>>();

        for (i, &pixel) in tmp.iter().cycle().skip(50 - rot.offset).take(50).enumerate() {
            self.pixels[rot.index][i] = pixel;
        }
    }

    fn rotate_col(&mut self, rot: &Rotation) {
        assert!(rot.index < 50);
        assert!(rot.offset <= 6);

        let tmp = self.pixels.iter().map(|r| r[rot.index]).collect::<Vec<_>>();

        for (i, &pixel) in tmp.iter().cycle().skip(6 - rot.offset).take(6).enumerate() {
            self.pixels[i][rot.index] = pixel;
        }
    }

    fn nb_pixel_on(&self) -> usize {
        self.pixels.iter().flat_map(|row| row.iter()).filter(|&p| *p == Pixel::On).count()
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            pixels: [[Pixel::Off; 50]; 6],
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                write!(f, "{}", match *pixel {
                    Pixel::On => '#',
                    Pixel::Off => ' ',
                })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

enum Action {
    Rect(Dimension),
    RotateRow(Rotation),
    RotateColumn(Rotation),
}

struct Dimension {
    width: usize,
    height: usize,
}

struct Rotation {
    index: usize,
    offset: usize,
}

fn parse_action(input: &str) -> Action {
    lazy_static! {
        static ref RE_RECT: Regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
        static ref RE_ROTATE: Regex = Regex::new(r"^rotate (row|column) [xy]=(\d+) by (\d+)$").unwrap();
    }

    if let Some(capture) = RE_RECT.captures(input) {
        let width = capture.at(1).unwrap().parse().unwrap();
        let height = capture.at(2).unwrap().parse().unwrap();

        Action::Rect(Dimension {
            width: width,
            height: height,
        })
    } else if let Some(capture) = RE_ROTATE.captures(input) {
        let target = capture.at(1).unwrap();
        let index = capture.at(2).unwrap().parse().unwrap();
        let offset = capture.at(3).unwrap().parse().unwrap();

        let rot = Rotation {
            index: index,
            offset: offset,
        };

        match target {
            "row" => Action::RotateRow(rot),
            "column" => Action::RotateColumn(rot),
            _ => unreachable!()
        }
    } else {
        unreachable!()
    }
}

pub fn run(input: &str) -> String {
    let mut screen = Screen::default();

    for line in input.lines() {
        let action = parse_action(line.trim());

        match action {
            Action::Rect(dim) => screen.rect(&dim),
            Action::RotateRow(rot) => screen.rotate_row(&rot),
            Action::RotateColumn(rot) => screen.rotate_col(&rot),
        }
    }

    format!("{}", screen)
}

pub fn challenge() -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day8-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}