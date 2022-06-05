#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use core::num;
use std::cmp::*;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::io;
use std::{io::prelude::*, io::*, str};

fn solve(lines: &mut Lines<StdinLock>) -> Result<()> {
    let mut out = BufWriter::new(stdout());
    let nums: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|ch| ch.parse::<i64>().unwrap())
        .collect();
    let (mut n, mut a, b) = (nums[0], nums[1], nums[2]);

    if !(a <= n / 2 && b >= n / 2 + 1 || a == n / 2 + 1 && b == n / 2) {
        writeln!(out, "{}", -1)?;
        return Ok(());
    }

    let mut res = vec![0; n as usize];
    res[0] = a;
    res[n as usize - 1] = b;
    let mut left = 1 as usize;
    let mut right = n as usize - 2;
    let (mut left_val, mut right_val) = (n, 1);

    let (mut left_min, mut right_max) = (a, b);

    let mid = (n / 2) as usize;
    while left < mid {
        if left_val == a || left_val == b {
            left_val -= 1;
        }
        res[left] = left_val;
        left_min = left_min.min(left_val);
        left += 1;
        left_val -= 1;
    }

    while mid <= right {
        if right_val == a || right_val == b {
            right_val += 1;
        }
        res[right] = right_val;
        right_max = right_max.max(right_val);
        right -= 1;
        right_val += 1;
    }

    if left_min != a || right_max != b {
        writeln!(out, "{}", -1)?;
        return Ok(());
    }

    for val in res {
        write!(out, "{} ", val)?;
    }
    writeln!(out)?;

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
