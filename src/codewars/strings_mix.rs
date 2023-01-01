use std::{
    collections::{BinaryHeap, HashMap},
    iter::{self, FromIterator},
};

#[derive(Ord, PartialEq, Eq)]
struct Pair {
    pos: i32,
    count: usize,
    chars: String,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.count > other.count {
            Some(std::cmp::Ordering::Greater)
        } else if self.count < other.count {
            Some(std::cmp::Ordering::Less)
        } else {
            if self.pos < other.pos {
                Some(std::cmp::Ordering::Greater)
            } else if self.pos > other.pos {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(self.chars.cmp(&other.chars).reverse())
            }
        }
    }
}

fn mix(s1: &str, s2: &str) -> String {
    let s1 = s1
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .collect::<Vec<_>>();
    let s2 = s2
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .collect::<Vec<_>>();
    let s1_collect = s1.into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let s2_collect = s2.into_iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut binary_heap = BinaryHeap::new();
    for (&key, &val) in &s1_collect {
        if val <= 1 {
            continue;
        }
        if !s2_collect.contains_key(&key) {
            binary_heap.push(Pair {
                pos: 1,
                count: val as usize,
                chars: String::from_iter(iter::repeat(key).take(val)),
            });
        } else {
            let s2_val = s2_collect.get(&key).unwrap();
            if val > *s2_val {
                binary_heap.push(Pair {
                    pos: 1,
                    count: val as usize,
                    chars: String::from_iter(iter::repeat(key).take(val)),
                });
            } else if val < *s2_val {
                binary_heap.push(Pair {
                    pos: 2,
                    count: *s2_val as usize,
                    chars: String::from_iter(iter::repeat(key).take(*s2_val)),
                });
            } else {
                binary_heap.push(Pair {
                    pos: 3,
                    count: val as usize,
                    chars: String::from_iter(iter::repeat(key).take(val)),
                });
            }
        }
    }
    for (key, val) in s2_collect {
        if val <= 1 {
            continue;
        }
        if !s1_collect.contains_key(&key) || *s1_collect.get(&key).unwrap() <= 1 {
            binary_heap.push(Pair {
                pos: 2,
                count: val as usize,
                chars: String::from_iter(iter::repeat(key).take(val)),
            });
        }
    }
    let mut res = String::new();
    while !binary_heap.is_empty() {
        let pair = binary_heap.pop().unwrap();
        if pair.pos == 1 {
            res.push_str(&format!("1:{}", pair.chars));
        } else if pair.pos == 2 {
            res.push_str(&format!("2:{}", pair.chars));
        } else {
            res.push_str(&format!("=:{}", pair.chars));
        }
        res.push_str("/");
    }
    if res.is_empty() {
        res
    } else {
        res[..res.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::mix;

    #[test]
    fn basics_mix() {
        testing(
            "Are they here",
            "yes, they are here",
            "2:eeeee/2:yy/=:hh/=:rr",
        );
        testing(
            "looping is fun but dangerous",
            "less dangerous than coding",
            "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg",
        );
        testing(
            " In many languages",
            " there's a pair of functions",
            "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt",
        );
        testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
        testing("codewars", "codewars", "");
        testing(
            "A generation must confront the looming ",
            "codewarrs",
            "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr",
        );
    }

    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        assert_eq!(&mix(s1, s2), exp)
    }
}
