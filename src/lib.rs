use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Iter;

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Histogram<T:Hash+Clone+Eq> {
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

    pub fn iter(&self) -> Iter<T,usize> {
        self.histogram.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_simple<'a>() -> Histogram<&'a str> {
        let mut h = Histogram::new();
        for s in ["a", "b", "a", "c", "a", "b"].iter() {
            h.bump(s);
        }
        h
    }

    #[test]
    fn it_works() {
        let h = make_simple();
        for (s, c) in [("a", 3), ("b", 2), ("c", 1), ("d", 0)].iter() {
            assert_eq!(h.count(s), *c);
        }
    }

    #[test]
    fn iterator() {
        let h = make_simple();
        let mut itered: Vec<_> = h.iter().map(|(s,c)| (*s, *c)).collect();
        itered.sort();
        assert_eq!(itered, vec![("a", 3), ("b", 2), ("c", 1)]);
    }
}
