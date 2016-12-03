struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Triangle {
            a: a,
            b: b,
            c: c,
        }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c &&
        self.a + self.c > self.b &&
        self.b + self.c > self.a
    }
}

pub fn run(input: &str) -> usize {
    let mut nb_valid = 0;

    for line in input.lines() {
        let triangle_str = line.trim().split_whitespace().collect::<Vec<_>>();
        assert!(triangle_str.len() == 3);

        let triangle = Triangle::new(
            triangle_str[0].parse().unwrap(),
            triangle_str[1].parse().unwrap(),
            triangle_str[2].parse().unwrap(),
        );

        if triangle.is_valid() {
            nb_valid += 1;
        }
    }

    nb_valid
}

pub fn challenge() -> usize {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day3-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn parsed_sample() {
    let triangle = Triangle::new(5, 10, 25);

    assert!(!triangle.is_valid());
}

#[test]
fn parsed_valid() {
    let triangle = Triangle::new(5, 5, 5);

    assert!(triangle.is_valid());
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        assert_eq!(super::run("5 10 25"), 0);
    }

    #[test]
    fn valid() {
        assert_eq!(super::run("5 5 5"), 1);
    }
}
