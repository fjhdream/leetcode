use std::collections::{HashMap, HashSet};

pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut map = HashMap::new();

    for num in nums1.into_iter().collect::<HashSet<_>>() {
        *map.entry(num).or_insert(0) += 1;
    }

    for num in nums2.into_iter().collect::<HashSet<_>>() {
        *map.entry(num).or_insert(0) += 1;
    }

    for num in nums3.into_iter().collect::<HashSet<_>>() {
        *map.entry(num).or_insert(0) += 1;
    }

    for (key, val) in map {
        if val >= 2 {
            res.push(key);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_out_of_three() {
        assert_eq!(
            two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
            vec![3, 2]
        );
        assert_eq!(
            two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
            vec![2, 3, 1]
        );
        assert_eq!(
            two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]),
            vec![]
        );
    }
}
