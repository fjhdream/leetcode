pub fn min_max_game(mut nums: Vec<i32>) -> i32 {
    while nums.len() > 1 {
        nums = nums
            .chunks(2)
            .enumerate()
            .map(|(idx, w)| {
                if idx & 1 == 0 {
                    w[0].min(w[1])
                } else {
                    w[0].max(w[1])
                }
            })
            .collect();
    }
    nums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_game() {
        assert_eq!(min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
        assert_eq!(min_max_game(vec![3]), 3);
    }

    #[test]
    fn test1() {
        assert_eq!(min_max_game(vec![70, 38, 21, 22]), 22);
    }
}
