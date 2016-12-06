use std::collections::HashMap;

struct Corrector {
    message: Option<Vec<HashMap<char, usize>>>,
}

impl Corrector {
    fn add_signal(&mut self, signal: &str) {
        let signal_chars = signal.chars();
        if let Some(ref mut message) = self.message {
            assert_eq!(message.len(), signal.len());

            for (c, hash_map) in signal_chars.zip(message.iter_mut()) {
                let count = hash_map.get(&c).unwrap_or(&0) + 1;
                hash_map.insert(c, count);
            }
        } else {
            let mut message = Vec::with_capacity(signal.len());
            for c in signal_chars {
                let mut hash_map = HashMap::new();
                hash_map.insert(c, 1);
                message.push(hash_map);
            }
            self.message = Some(message);
        }
    }

    fn correct(&self) -> String {
        let message = self.message.as_ref().unwrap();

        fn less_frequent_char(hash_map: &HashMap<char, usize>) -> char {
            let mut min = None;
            for (&c, &count) in hash_map.iter() {
                if let Some((_, min_count)) = min {
                    if count < min_count {
                        min = Some((c, count));
                    }
                } else {
                    min = Some((c, count));
                }
            }
            min.unwrap().0
        }
        message.iter().map(less_frequent_char).collect()
    }
}

impl Default for Corrector {
    fn default() -> Self {
        Corrector {
            message: None,
        }
    }
}

fn run(input: &str) -> String {
    let mut corrector = Corrector::default();

    for line in input.lines() {
        corrector.add_signal(line.trim());
    }

    corrector.correct()
}

pub fn challenge() -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day6-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn sample() {
    let input = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";
    assert_eq!(run(input), "advent");
}