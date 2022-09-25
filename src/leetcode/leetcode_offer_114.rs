use std::collections::{HashMap, HashSet, VecDeque};

pub fn alien_order(words: Vec<String>) -> String {
    let mut graph = HashMap::new();
    let mut ingree = HashMap::new();
    for word in words.iter() {
        for ch in word.chars() {
            ingree.entry(ch).or_insert(0);
        }
    }

    for i in 0..words.len() - 1 {
        let word1 = &words[i];
        let word2 = &words[i + 1];
        let mut j = 0;
        let word1_chs = word1.chars().collect::<Vec<_>>();
        let word2_chs = word2.chars().collect::<Vec<_>>();
        while j < word1.len() && j < word2.len() {
            let ch1 = word1_chs[j];
            let ch2 = word2_chs[j];
            if ch1 != ch2 {
                graph.entry(ch1).or_insert(vec![]).push(ch2);
                *ingree.entry(ch2).or_insert(0) += 1;
                break;
            }
            j += 1;
        }
        if j == word1.len() || j == word2.len() {
            if word1.len() > word2.len() {
                return "".to_string();
            }
        }
    }

    // bfs
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut res = String::new();
    for (ch, size) in ingree.iter() {
        if *size == 0 && !visited.contains(ch) {
            queue.push_back(*ch);
        }
    }
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        res.push(node);
        visited.insert(node);
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                ingree.entry(*neighbor).and_modify(|e| *e -= 1);
                if visited.contains(neighbor) {
                    return "".to_string();
                }
                if ingree[neighbor] == 0 {
                    queue.push_back(*neighbor);
                }
            }
        } else {
            // no neighbors
            continue;
        }
    }
    return if res.len() != ingree.len() {
        "".to_string()
    } else {
        res
    };
}

#[test]
fn test() {
    let res = alien_order(vec![
        "wrt".to_string(),
        "wrf".to_string(),
        "er".to_string(),
        "ett".to_string(),
        "rftt".to_string(),
    ]);
    assert_eq!(res, "wertf".to_string());
}

#[test]
fn test2() {
    let res = alien_order(vec!["z".to_string(), "x".to_string()]);
    assert_eq!(res, "zx".to_string());
}

#[test]
fn test3() {
    let res = alien_order(vec!["z".to_string(), "x".to_string(), "z".to_string()]);
    assert_eq!(res, "".to_string());
}

#[test]
fn test4() {
    let res = alien_order(vec!["z".to_string(), "z".to_string()]);
    assert_eq!(res, "z".to_string());
}

#[test]
fn test5() {
    let res = alien_order(vec!["ab".to_string(), "adc".to_string()]);
    assert_eq!(res, "abcd".to_string());
}

#[test]
fn test6() {
    let res = alien_order(vec!["abc".to_string(), "ab".to_string()]);
    assert_eq!(res, "".to_string());
}
