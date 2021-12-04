use core::fmt;
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

    pub fn ranking(&self) -> Vec<T> {
        let mut ranking: Vec<(usize,T)> = self.iter().map(|(t, n)| (*n, t.clone())).collect();
        ranking.sort_by_key(|(n,_)| -(*n as isize));
        ranking.iter().map(|(_,t)| t.clone()).collect()
    }

    pub fn mode(&self) -> Option<T> {
        self.iter().max_by_key(|(_,count)| **count).map(|(key, _)| key.clone())
    }

    pub fn total_count(&self) -> usize {
        self.iter().map(|(_,value)| value).sum()
    }
}

impl<K: Hash + Eq + Copy + std::cmp::Ord + fmt::Display> fmt::Display for Histogram<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut in_order: Vec<K> = self.iter().map(|(k,_)| k).copied().collect();
        in_order.sort();
        for label in in_order {
            write!(f, "{}:{}; ", label, self.count(&label))?;
        }
        Ok(())
    }
}

pub fn mode<K: Eq + Copy + Hash, I: Iterator<Item=K>>(items: &mut I) -> K {
    let mut counts = Histogram::new();
    for k in items {
        counts.bump(&k);
    }
    counts.mode().unwrap()
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

    #[test]
    fn sorting() {
        let h = make_simple();
        let ranking = h.ranking();
        assert_eq!(ranking, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_mode() {
        let nums = vec![5, 4, 3, 4, 5, 6, 5];
        assert_eq!(5, *mode(&mut nums.iter()));
    }
}
