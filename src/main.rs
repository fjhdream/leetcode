#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
use core::num;
use std::cmp::*;
use std::collections::{VecDeque, BinaryHeap, HashMap};
use std::io;
use std::{io::prelude::*, io::*, str};

fn solve(lines: &mut Lines<StdinLock>) -> Result<()> {
    let mut out = BufWriter::new(stdout());
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|ch| ch.parse::<i32>().unwrap() )
        .collect::<>();
    let (mut n, mut a, b) = (nums[0], nums[1], nums[2]);
    let mut ans = vec!['a'; n as usize];
    let (left, right) = (0, 0);
    let mut distinct = 0;
    let mut cnt = [0; 26];
    while right < n {
        if distinct < b {
            cnt[]
        }
    }
    
    Ok(())
}


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let tc = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    for _ in 0..tc {
        solve(&mut lines)?;
    }
    Ok(())
}
