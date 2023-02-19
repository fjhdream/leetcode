use std::collections::HashSet;
pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut user_map = std::collections::HashMap::new();
    for log in logs {
        let user_id = log[0];
        let minute = log[1];
        let user_minutes = user_map.entry(user_id).or_insert(HashSet::new());
        user_minutes.insert(minute);
    }
    let mut result = vec![0; k as usize];
    for (_, minutes) in user_map {
        result[minutes.len() - 1] += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding_users_active_minutes() {
        assert_eq!(
            finding_users_active_minutes(
                vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]],
                5
            ),
            vec![0, 2, 0, 0, 0]
        );
        assert_eq!(
            finding_users_active_minutes(vec![vec![1, 1], vec![2, 2], vec![2, 3]], 4),
            vec![1, 1, 0, 0]
        );
    }
}
