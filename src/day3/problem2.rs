struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    fn new(a: usize, b: usize, c: usize) -> Self {
        Triangle { a: a, b: b, c: c }
    }

    fn is_valid(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

pub fn run(input: &str) -> usize {
    let mut nb_valid = 0;

    let mut index = 0;
    let mut cols = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    for line in input.lines() {
        let input_str = line.trim().split_whitespace().collect::<Vec<_>>();
        assert!(input_str.len() == 3);

        for (i, mut col) in cols.iter_mut().enumerate() {
            col[index] = input_str[i].parse().unwrap();
        }

        index += 1;

        if index % 3 == 0 {
            index = 0;

            for col in cols.iter() {
                let triangle = Triangle::new(col[0], col[1], col[2]);

                if triangle.is_valid() {
                    nb_valid += 1;
                }
            }
        }
    }

    assert!(index == 0);

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
        assert_eq!(super::run("101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 \
                               602\n203 403 603"),
                   6);
    }
}
