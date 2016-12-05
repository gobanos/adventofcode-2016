use crypto::md5::Md5;
use crypto::digest::Digest;

struct WarGame {
    id: String,
    index: usize,
    password: String,
}

impl WarGame {
    fn new(id: &str) -> Self {
        WarGame {
            id: id.into(),
            index: 0,
            password: String::with_capacity(8),
        }
    }

    fn md5(&self) -> String {
        let to_hash = format!("{}{}", self.id, self.index);
        let mut digest = Md5::new();

        digest.input_str(&to_hash);

        digest.result_str()
    }
}