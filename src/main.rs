#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use core::num;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::io;
use std::{io::prelude::*, io::*, str};

#[derive(Debug, Clone, Eq, PartialEq)]
struct tree_array {
    trees: Vec<i64>,
    n: usize,
}

impl tree_array {
    fn new(n: usize) -> Self {
        tree_array {
            n,
            trees: vec![0; n + 1],
        }
    }

    fn low_bit(x: i64) -> i64 {
        x & -x
    }

    fn add(&mut self, x: usize, data: i64) {
        let mut i = x + 1;
        while i <= self.n {
            self.trees[i] += data;
            i += Self::low_bit(i as i64) as usize;
        }
    }

    fn query(&self, x: usize) -> i64 {
        let mut i = x + 1;
        let mut res = 0;
        while i > 0 {
            res += self.trees[i];
            let diff = Self::low_bit(i as i64);
            if diff >= i as i64 {
                break;
            }
            i -= diff as usize;
        }
        res
    }
}

fn solve(lines: &mut Lines<StdinLock>) -> Result<()> {
    let mut out = BufWriter::new(stdout());
    let nums: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|ch| ch.parse::<i64>().unwrap())
        .collect();
    let n = nums[0] as usize;
    let str = lines.next().unwrap().unwrap();
    let mut str_chs = str.chars().collect::<Vec<_>>();
    let mut res = 0i64;
    let mut prefix_minus_num = vec![0i64; n + 1];
    for (i, ch) in str_chs.iter().enumerate() {
        prefix_minus_num[i + 1] = prefix_minus_num[i] + if *ch == '-' { 1 } else { -1 };
    }
    for m in 0..3 {
        let mut tree_array = tree_array::new(2 * n + 1);
        for i in 0..=n {
            if (prefix_minus_num[i] - m) % 3 == 0 {
                res += tree_array.query((prefix_minus_num[i] + n as i64) as usize);
                tree_array.add((n as i64 + prefix_minus_num[i]) as usize, 1);
            }
        }
    }
    writeln!(out, "{}", res)?;
    Ok(())
}

fn main() -> Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    let tc = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..tc {
        solve(&mut lines)?;
    }
    Ok(())
}
