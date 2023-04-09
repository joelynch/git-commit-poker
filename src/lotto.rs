use std::collections::BTreeMap;

use num_bigint::BigInt;
use num_integer::binomial;
use num_rational::BigRational;
use num_traits::{cast::ToPrimitive, Pow};

pub struct LottoResult<'a> {
    pub hash: &'a str,
    pub rules: Vec<Box<dyn LottoRuleFamily<'a> + 'a>>,
}

impl<'a> LottoResult<'a> {
    pub fn new(hash: &'a str) -> Self {
        let mut rules: Vec<Box<dyn LottoRuleFamily<'a> + 'a>> = vec![];
        if let Some(rule) = NOfAKind::new(hash) {
            rules.push(Box::new(rule));
        }
        if let Some(rule) = Flush::new(hash) {
            rules.push(Box::new(rule));
        }
        if let Some(rule) = Straight::new(hash) {
            rules.push(Box::new(rule));
        }
        rules.sort_by_key(|r| -(r.points() as i64));
        Self { hash, rules }
    }

    pub fn total_points(&self) -> u64 {
        self.rules.iter().map(|r| r.points()).sum()
    }
}

pub trait LottoRuleFamily<'a> {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn probability(&self) -> f64;
    fn points(&self) -> u64 {
        (100.0 / self.probability()).ceil() as u64
    }
    fn positions(&self) -> Vec<Vec<usize>>;
}

pub struct NOfAKind<'a> {
    commit: &'a str,
    pub values: BTreeMap<char, usize>,
}

impl<'a> NOfAKind<'a> {
    pub fn new(commit: &'a str) -> Option<Self> {
        let values: BTreeMap<char, usize> = commit
            .chars()
            .fold(BTreeMap::new(), |mut hash, c| {
                *hash.entry(c).or_insert(0) += 1;
                hash
            })
            .into_iter()
            .filter(|(_, v)| *v > 1)
            .collect();
        if values.is_empty() {
            return None;
        }
        Some(NOfAKind { commit, values })
    }
}

impl<'a> LottoRuleFamily<'a> for NOfAKind<'a> {
    fn name(&self) -> String {
        if self.values.is_empty() {
            return "N/A".into();
        }
        let val = self.values.values().next().unwrap();
        if self.values.len() == 1 {
            format!("{} of a kind", self.values.values().next().unwrap())
        } else if self.values.values().all(|v| *v == 2) {
            format!("{} pairs!", self.values.len())
        } else if self.values.values().all(|v| v == val) {
            format!("{} x {} of a kind!", self.values.len(), val)
        } else {
            "FULL HOUSE!!".into()
        }
    }

    fn description(&self) -> String {
        let mut iter = self.values.iter();
        let (k, v) = iter.next().unwrap();
        let mut s = format!("{} x {}", v, k);
        for (k, v) in iter {
            s.push_str(&format!(", {} x {}", v, k));
        }
        s
    }

    fn probability(&self) -> f64 {
        if self.values.is_empty() {
            return 0.0;
        }
        let numerator = self
            .values
            .values()
            .map(|v| binomial(BigInt::from(self.commit.len()), BigInt::from(*v)))
            .fold(BigInt::from(1), |acc, x| acc * x)
            * BigInt::from(15)
                .pow((self.commit.len() - self.values.values().sum::<usize>()) as u32);
        let denominator = BigInt::from(16).pow(self.commit.len() as u32);
        BigRational::new(numerator, denominator).to_f64().unwrap()
    }

    fn positions(&self) -> Vec<Vec<usize>> {
        let mut positions = vec![];
        let mut values: Vec<_> = self.values.iter().collect();
        values.sort_by_key(|(_, v)| -(**v as i64));
        for (k, _) in self.values.iter() {
            let mut pos = vec![];
            for (i, c) in self.commit.chars().enumerate() {
                if c == *k {
                    pos.push(i);
                }
            }
            positions.push(pos);
        }
        positions
    }
}

pub struct Flush<'a> {
    commit: &'a str,
    letters: bool,
}

impl<'a> Flush<'a> {
    pub fn new(commit: &'a str) -> Option<Self> {
        if commit.chars().all(|c| c.is_ascii_alphabetic()) {
            Some(Flush {
                commit,
                letters: true,
            })
        } else if commit.chars().all(|c| c.is_ascii_digit()) {
            Some(Flush {
                commit,
                letters: false,
            })
        } else {
            None
        }
    }
}

impl<'a> LottoRuleFamily<'a> for Flush<'a> {
    fn name(&self) -> String {
        "Flush".into()
    }

    fn description(&self) -> String {
        if self.letters {
            "all letters".into()
        } else {
            "all numbers".into()
        }
    }

    fn probability(&self) -> f64 {
        if self.letters {
            (10.0 / 16.0).pow(self.commit.len() as f64)
        } else {
            (6.0 / 16.0).pow(self.commit.len() as f64)
        }
    }

    fn positions(&self) -> Vec<Vec<usize>> {
        let mut positions = vec![];
        for (pos, char) in self.commit.chars().enumerate() {
            if (self.letters && char.is_ascii_alphabetic())
                || (!self.letters && char.is_ascii_digit())
            {
                positions.push(pos);
            }
        }
        vec![positions]
    }
}

struct Straight<'a> {
    commit: &'a str,
    run: String,
}

static MIN_STRAIGHT: u16 = 4;

impl<'a> Straight<'a> {
    pub fn new(commit: &'a str) -> Option<Self> {
        let mut sorted = commit.chars().collect::<Vec<char>>();
        sorted.sort();
        let mut iter = sorted.into_iter();
        let mut last = iter.next().unwrap();
        let mut longest_run = String::new();
        let mut current_run = String::from(last);

        for c in iter {
            if Straight::to_int(c) == Straight::to_int(last) + 1 {
                current_run.push(c as u8 as char);
            } else {
                if current_run.len() >= longest_run.len() {
                    longest_run = current_run;
                }
                current_run = String::from(c);
            }
            last = c;
        }

        if current_run.len() > longest_run.len() {
            longest_run = current_run;
        }

        if longest_run.len() >= MIN_STRAIGHT as usize {
            Some(Straight {
                commit,
                run: longest_run,
            })
        } else {
            None
        }
    }

    fn to_int(c: char) -> u16 {
        if c.is_ascii_digit() {
            c.to_digit(10).unwrap() as u16
        } else {
            c.to_ascii_lowercase() as u16 - 87
        }
    }
}

impl<'a> LottoRuleFamily<'a> for Straight<'a> {
    fn name(&self) -> String {
        if self.commit.len() == self.run.len() {
            "Straight".into()
        } else {
            "Partial straight".into()
        }
    }

    fn description(&self) -> String {
        self.run.clone()
    }

    fn probability(&self) -> f64 {
        let numerator = // which chars contain the straight
            permutations(
                self.commit.len() as u32,
                self.run.len() as u32)
            // chars don't contain the straight can be anything
            * BigInt::from(16).pow((self.commit.len() - self.run.len()) as u32)
            // first char choices
            * (16 - self.run.len());
        // straight ordering

        let denominator = BigInt::from(16).pow(self.commit.len() as u32);
        BigRational::new(numerator, denominator).to_f64().unwrap()
    }

    fn positions(&self) -> Vec<Vec<usize>> {
        let mut positions = vec![];
        for char in self.run.chars() {
            for (pos, c) in self.commit.chars().enumerate() {
                if c == char {
                    positions.push(pos);
                    break;
                }
            }
        }
        vec![positions]
    }
}

fn permutations(n: u32, k: u32) -> BigInt {
    let mut n = n;
    if n == k {
        return BigInt::from(1);
    }
    let mut result = BigInt::from(n);
    n -= 1;
    while n != k {
        result *= n;
        n -= 1;
    }
    result
}

#[cfg(test)]
mod test {
    use num_traits::abs;

    use super::*;

    #[test]
    fn test_n_of_a_kind() {
        let commit = "aabbccddeeff";
        let rule = NOfAKind::new(commit).unwrap();
        assert_eq!(rule.name(), "6 pairs!");
        assert_eq!(rule.description(), "2 as, 2 bs, 2 cs, 2 ds, 2 es, 2 fs");
        assert!(
            abs(rule.probability() - 0.00029364581882) < 0.00000001,
            "{}",
            rule.probability()
        );
        assert_eq!(rule.points(), 340547);
        let mut pos = rule.positions();
        pos.sort();
        assert_eq!(
            pos,
            vec![
                vec![0, 1],
                vec![2, 3],
                vec![4, 5],
                vec![6, 7],
                vec![8, 9],
                vec![10, 11]
            ]
        );
    }

    #[test]
    fn test_flush() {
        let commit = "abcdef";
        let rule = Flush::new(commit).unwrap();
        assert_eq!(rule.name(), "Flush");
        assert_eq!(rule.description(), "All letters");
        assert!(
            abs(rule.probability() - 0.05960464477) < 0.00000001,
            "{}",
            rule.probability()
        );
        assert_eq!(rule.points(), 1678);
        let pos = rule.positions();
        assert_eq!(pos, vec![vec![0, 1, 2, 3, 4, 5],]);
    }

    #[test]
    fn test_straight() {
        let commit = "abcdef";
        let rule = Straight::new(commit).unwrap();
        assert_eq!(rule.name(), "Straight");
        assert_eq!(rule.description(), "abcdef");
        assert!(
            abs(rule.probability() - 0.0000005960464477539063) < 0.00000000001,
            "{}",
            rule.probability()
        );
        assert_eq!(rule.points(), 167772160);
        let pos = rule.positions();
        assert_eq!(pos, vec![vec![0, 1, 2, 3, 4, 5],]);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(5, 3), BigInt::from(20));
        assert_eq!(permutations(5, 5), BigInt::from(1));
    }
}
