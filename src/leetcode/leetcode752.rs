use std::collections::HashSet;
use std::iter::FromIterator;
use std::mem::swap;

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited:HashSet<String> = HashSet::from_iter(deadends.into_iter());
    let mut start = String::from("0000");

    let mut res = 0;
    let mut set1 = HashSet::new();
    set1.insert(start);
    let mut set2 = HashSet::new();
    set2.insert(target);

    while !set1.is_empty() && !set2.is_empty() {
        if set1.len() > set2.len() {
            swap(&mut set1, &mut set2);
        }

        let mut temp = HashSet::new();
        for solution in set1.iter() {
            if visited.contains(solution) {
                continue;
            }

            if set2.contains(solution) {
                return res;
            }

            visited.insert(solution.clone());

            for i in 0..4 {
                temp.insert(plus_one(solution.clone(), i));
                temp.insert(min_one(solution.clone(),i));
            }

        }
        swap(&mut set1, &mut temp);

        res += 1;
    }
    -1
}

fn plus_one(lock: String, i: usize) -> String {
    let mut chars: Vec<char> = lock.chars().collect();
    let char = chars[i];
    if char == '9' {
        chars[i] = '0' ;
    } else {
        chars[i] = char::from(chars[i] as u8 + 1);
    }

    chars.iter().collect()
}

fn min_one(mut lock: String, i: usize) -> String {
    let mut chars: Vec<char> = lock.chars().collect();
    let char = chars[i];
    if char == '0' {
        chars[i] = '9' ;
    } else {
        chars[i] = char::from(chars[i] as u8 - 1);
    }

    chars.iter().collect()
}

#[test]
fn test_example() {
    let deadends: Vec<String> = vec!["0201".to_string(),"0101".to_string(),"0102".to_string(),"1212".to_string(),"2002".to_string()];
    let target = "0202".to_string();
    let ans = open_lock(deadends, target);
    assert_eq!(ans, 6);
}