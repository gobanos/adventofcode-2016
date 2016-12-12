use regex::Regex;

macro_rules! make_sized_screen {
    ($s:ident, width: $w:expr, height: $h:expr) => {
        use std::fmt;

        struct $s {
            pixels: [[Pixel; $w]; $h],
        }

        impl $s {
            fn rect(&mut self, dim: &Dimension) {
                assert!(dim.width <= $w);
                assert!(dim.height <= $h);

                for mut row in self.pixels[0..dim.height].iter_mut() {
                    for mut pixel in row[0..dim.width].iter_mut() {
                        *pixel = Pixel::On;
                    }
                }
            }

            fn rotate_row(&mut self, rot: &Rotation) {
                assert!(rot.index < $h);
                assert!(rot.offset <= $w);

                let tmp = self.pixels[rot.index].iter().map(|p| *p).collect::<Vec<_>>();

                for (i, &pixel) in tmp.iter().cycle().skip($w - rot.offset).take($w).enumerate() {
                    self.pixels[rot.index][i] = pixel;
                }
            }

            fn rotate_col(&mut self, rot: &Rotation) {
                assert!(rot.index < $w);
                assert!(rot.offset <= $h);

                let tmp = self.pixels.iter().map(|r| r[rot.index]).collect::<Vec<_>>();

                for (i, &pixel) in tmp.iter().cycle().skip($h - rot.offset).take($h).enumerate() {
                    self.pixels[i][rot.index] = pixel;
                }
            }

            fn nb_pixel_on(&self) -> usize {
                self.pixels.iter().flat_map(|row| row.iter()).filter(|&p| *p == Pixel::On).count()
            }
        }

        impl Default for $s {
            fn default() -> Self {
                $s {
                    pixels: [[Pixel::Off; $w]; $h],
                }
            }
        }

        impl fmt::Debug for $s {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for row in self.pixels.iter() {
                    for pixel in row.iter() {
                        write!(f, "{}", match *pixel {
                            Pixel::On => '#',
                            Pixel::Off => '.',
                        })?;
                    }
                    write!(f, "\n")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Pixel {
    On,
    Off,
}

make_sized_screen!(Screen, width: 50, height: 6);

pub enum Action {
    Rect(Dimension),
    RotateRow(Rotation),
    RotateColumn(Rotation),
}

pub struct Dimension {
    width: usize,
    height: usize,
}

pub struct Rotation {
    index: usize,
    offset: usize,
}

pub fn parse_action(input: &str) -> Action {
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

pub fn run(input: &str) -> usize {
    let mut screen = Screen::default();

    for line in input.lines() {
        let action = parse_action(line.trim());

        match action {
            Action::Rect(dim) => screen.rect(&dim),
            Action::RotateRow(rot) => screen.rotate_row(&rot),
            Action::RotateColumn(rot) => screen.rotate_col(&rot),
        }
    }

    screen.nb_pixel_on()
}

pub fn challenge() -> usize {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day8-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[cfg(test)]
mod test {
    make_sized_screen!(TestScreen, width: 7, height: 3);

    use super::*;

    fn run_action(screen: &mut TestScreen, action: &Action) {
        match *action {
            Action::Rect(ref dim) => screen.rect(dim),
            Action::RotateRow(ref rot) => screen.rotate_row(rot),
            Action::RotateColumn(ref rot) => screen.rotate_col(rot),
        }
    }

    #[test]
    fn sample() {
        let mut screen = TestScreen::default();

        run_action(&mut screen, &parse_action("rect 3x2"));
        assert_eq!(format!("{:?}", screen), "###....\n###....\n.......\n");

        run_action(&mut screen, &parse_action("rotate column x=1 by 1"));
        assert_eq!(format!("{:?}", screen), "#.#....\n###....\n.#.....\n");

        run_action(&mut screen, &parse_action("rotate row y=0 by 4"));
        assert_eq!(format!("{:?}", screen), "....#.#\n###....\n.#.....\n");

        run_action(&mut screen, &parse_action("rotate column x=1 by 1"));
        assert_eq!(format!("{:?}", screen), ".#..#.#\n#.#....\n.#.....\n");
    }
}