use std::collections::{HashMap, HashSet};

pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    let mut card_set = HashSet::new();
    for i in 0..suits.len() {
        card_set.insert(suits[i]);
    }
    if card_set.len() == 1 {
        return "flush".to_string();
    }
    let mut card_map = HashMap::new();
    for i in 0..ranks.len() {
        card_map
            .entry(ranks[i])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    if card_map.len() == 5 {
        return "High Card".to_string();
    }
    for (_, val) in card_map {
        if val > 2 {
            return "Three of a Kind".to_string();
        }
    }
    return "Pair".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_hand() {
        assert_eq!(
            best_hand(vec![1, 1, 2, 2, 3], vec!['a', 'b', 'c', 'd', 'e']),
            "Pair".to_string()
        );
        assert_eq!(
            best_hand(vec![1, 1, 1, 1, 3], vec!['a', 'b', 'c', 'd', 'e']),
            "flush".to_string()
        );
        assert_eq!(
            best_hand(vec![1, 2, 3, 4, 5], vec!['a', 'b', 'c', 'd', 'e']),
            "High Card".to_string()
        );
    }
}
