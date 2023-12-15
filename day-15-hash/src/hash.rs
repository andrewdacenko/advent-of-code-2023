use std::collections::HashMap;

type Cache = HashMap<(usize, usize), usize>;

struct Hasher {
    cache: Cache,
}

impl Hasher {
    pub fn new() -> Self {
        Hasher {
            cache: HashMap::new(),
        }
    }

    pub fn hash_string(&mut self, string: &str) -> usize {
        string
            .as_bytes()
            .iter()
            .fold(0 as usize, |acc, code| self.digest(&acc, &(*code as usize)))
    }

    fn digest(&mut self, acc: &usize, code: &usize) -> usize {
        if let Some(cached) = self.cache.get(&(*acc, *code)) {
            return *cached;
        }
        let res = (*acc as usize + *code as usize) * 17 % 256;
        self.cache.insert((*acc, *code), res);
        return res as usize;
    }
}

pub fn hash_string(string: &str) -> usize {
    let mut hasher = Hasher::new();
    string
        .split(",")
        .map(|line| hasher.hash_string(line) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compares_vecs() {
        assert_eq!(Hasher::new().hash_string("rn=1"), 30);
        assert_eq!(Hasher::new().hash_string("cm-"), 253);
        assert_eq!(Hasher::new().hash_string("qp=3"), 97);
        assert_eq!(Hasher::new().hash_string("cm=2"), 47);
        assert_eq!(Hasher::new().hash_string("qp-"), 14);
        assert_eq!(Hasher::new().hash_string("pc=4"), 180);
        assert_eq!(Hasher::new().hash_string("ot=9"), 9);
        assert_eq!(Hasher::new().hash_string("ab=5"), 197);
        assert_eq!(Hasher::new().hash_string("pc-"), 48);
        assert_eq!(Hasher::new().hash_string("pc=6"), 214);
        assert_eq!(Hasher::new().hash_string("ot=7"), 231);
    }
}
