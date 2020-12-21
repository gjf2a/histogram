use std::collections::HashMap;
use std::hash::Hash;

pub struct Histogram<T> {
    histogram: HashMap<T,usize>
}

impl <T:Hash+Clone+Eq> Histogram<T> {
    pub fn new() -> Self { Histogram { histogram: HashMap::new()}}

    pub fn bump(&mut self, item: &T) {
        match self.histogram.get_mut(item) {
            None => {self.histogram.insert(item.clone(), 1);}
            Some(count) => {*count += 1}
        };
    }

    pub fn count(&self, item: &T) -> usize {
        *self.histogram.get(item).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut h = Histogram::new();
        for s in ["a", "b", "a", "c", "a", "b"].iter() {
            h.bump(s);
        }
        for (s, c) in [("a", 3), ("b", 2), ("c", 1), ("d", 0)].iter() {
            assert_eq!(h.count(s), *c);
        }
    }
}
