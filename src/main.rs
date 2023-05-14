#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use core::num;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::iter::FromIterator;
use std::{io::prelude::*, io::*, str};

const MOD: i64 = 1_000_000_007;

fn solve(lines: &mut Lines<StdinLock>) -> Result<()> {
    let mut out = BufWriter::new(stdout());
    let inputs: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|ch| ch.parse::<i64>().unwrap())
        .collect();
    let (n, p) = (inputs[0], inputs[1]);
    let nums = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|ch| ch.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut duplicate_map: HashMap<i64, bool> =
        HashMap::from_iter(nums.iter().map(|&x| (x, false)));
    // 去除给定集合a 中的可以转换元素
    for &num in nums.iter().rev() {
        if !*duplicate_map.get(&num).unwrap() {
            let mut tmp = num;
            while tmp > 0 {
                let mut trans = false;
                if tmp & 1 == 1 {
                    trans = true;
                    tmp >>= 1;
                } else if tmp % 4 == 0 {
                    trans = true;
                    tmp >>= 2;
                } else {
                    break;
                }
                if trans && duplicate_map.contains_key(&tmp) {
                    duplicate_map.insert(num, true);
                    break;
                }
            }
        }
    }

    let mut f = vec![0i64; 200000 + 1];
    for (key, val) in duplicate_map.iter() {
        if !val {
            f[count_bit(*key)] += 1;
        }
    }

    let mut res = 0i64;
    f[1] = f[0] + f[1];
    f[2] = f[1] + f[0] + f[2];
    res = (res + f[2] + f[1]) % MOD;

    for i in 3..=p as usize {
        f[i] = (f[i] + f[i - 1] + f[i - 2]) % MOD;
        res = (res + f[i]) % MOD;
    }

    writeln!(out, "{}", res)?;
    Ok(())
}

fn count_bit(x: i64) -> usize {
    let mut res = 0;
    let mut tmp = x;
    while tmp > 0 {
        res += 1;
        tmp >>= 1;
    }
    res
}

fn main() -> Result<()> {
    let stdin = stdin();
    let mut lines = stdin.lock().lines();

    // let tc = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    // for _ in 0..tc {
    solve(&mut lines)?;
    // }
    Ok(())
}
