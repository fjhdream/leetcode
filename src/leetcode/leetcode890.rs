pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    let mut res = Vec::new();
    let pattern_chs = pattern.chars().collect::<Vec<_>>();
    for word in words {
        let mut pattern_to_world_map = vec![-1; 26];
        let mut world_to_pattern_map = vec![-1; 26];
        if word.len() != pattern.len() {
            continue;
        }

        let mut mapped = true;
        for (i, ch) in word.chars().enumerate() {
            let pattern_idx = char_index(pattern_chs[i]);
            let word_idx = char_index(ch);
            if pattern_to_world_map[pattern_idx as usize] == -1
                && world_to_pattern_map[word_idx as usize] == -1
            {
                pattern_to_world_map[pattern_idx as usize] = word_idx;
                world_to_pattern_map[word_idx as usize] = pattern_idx;
            } else {
                if word_idx != pattern_to_world_map[pattern_idx as usize]
                    || pattern_idx != world_to_pattern_map[word_idx as usize]
                {
                    mapped = false;
                    break;
                }
            }
        }

        if mapped {
            res.push(word);
        }
    }
    res
}

fn char_index(ch: char) -> i32 {
    (ch as u8 - 'a' as u8) as i32
}

#[test]
fn test() {
    let ans = find_and_replace_pattern(
        vec![
            "abc".to_string(),
            "deq".to_string(),
            "mee".to_string(),
            "aqq".to_string(),
            "dkd".to_string(),
            "ccc".to_string(),
        ],
        "abb".to_string(),
    );
    assert_eq!(ans, vec!["mee".to_string(), "aqq".to_string()]);
}
