use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let neighbors: Vec<Vec<usize>> = vec![vec![1,3],vec![0,2,4],vec![1,5], vec![0,4], vec![1,3,5], vec![2,4]];
    let initial = form_string(board);
    if "123450".eq(&initial) {
        return 0;
    }

    let mut step = 0;
    let mut queue = VecDeque::new();
    queue.push_back(initial.to_string());
    let mut visited = HashSet::new();
    visited.insert(initial.clone());
    while !queue.is_empty() {
        step += 1;
        let size = queue.len();
        for i in 0..size {
            let status = queue.pop_front().unwrap();
            for next_status in get_status(&neighbors, status).iter() {
                if !visited.contains(next_status) {
                    if "123450".eq(next_status) {
                        return step;
                    }
                    queue.push_back(next_status.clone());
                    visited.insert(next_status.clone());
                }
            }
        }
    }
    -1
}

fn form_string(board: Vec<Vec<i32>>) -> String {
    let mut chars = vec![];
    for i in 0..2 {
        for j in 0..3 {
            let unit = char::from_digit(board[i][j] as u32, 10).unwrap();
            chars.push(unit);
        }
    }

    String::from_iter(chars.iter())
}

fn get_status(neighbors: &Vec<Vec<usize>>, status: String) -> Vec<String> {
    let mut ans:Vec<String> = Vec::new();
    let mut chars: Vec<char> = status.chars().collect();
    let x = status.find("0").unwrap();
    for &y in neighbors[x].iter(){
        chars.swap(x, y);
        ans.push(chars.iter().collect());
        chars.swap(x, y);
    }
    ans
}

#[test]
fn test_example() {
    let board = vec![vec![4, 1, 2], vec![5, 0, 3]];
    let ans = sliding_puzzle(board);
    assert_eq!(ans, 5);
    let str = "123540".to_string();
    assert_eq!(str.find('0').unwrap(), 5);
}