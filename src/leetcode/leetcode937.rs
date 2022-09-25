pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
    logs.sort_by(log_compare);
    return logs;
}

fn log_compare(log1: &String, log2: &String) -> std::cmp::Ordering {
    let (log1_id, log1_rest) = log1.split_once(' ').unwrap();
    let (log2_id, log2_rest) = log2.split_once(' ').unwrap();
    if log1_rest.starts_with(is_ascii_digit) && log2_rest.starts_with(is_ascii_digit) {
        std::cmp::Ordering::Equal
    } else if log1_rest.starts_with(is_ascii_digit) {
        std::cmp::Ordering::Greater
    } else if log2_rest.starts_with(is_ascii_digit) {
        std::cmp::Ordering::Less
    } else if log1_rest.cmp(log2_rest) == std::cmp::Ordering::Equal {
        log1_id.cmp(log2_id)
    } else {
        log1_rest.cmp(log2_rest) 
    }
}

fn is_ascii_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}


#[test]
fn test_leetcode937() {
    assert_eq!(
        vec!["g1 act car","a8 act zoo","ab1 off key dog","a1 9 2 3 1","zo4 4 7"],
        reorder_log_files(vec![
            "a8 act zoo".to_string(),
            "ab1 off key dog".to_string(),
            "a1 9 2 3 1".to_string(),
            "g1 act car".to_string(),
            "zo4 4 7".to_string(),
        ])
    );
}

#[test]
fn test() {
    println!("{:?}", "act car".cmp("act zoo"))
}