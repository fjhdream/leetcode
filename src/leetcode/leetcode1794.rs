use std::collections::HashMap;

pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
    let nums_len = nums.len();
    let groups_len = groups.iter().map(|group| group.len()).sum::<usize>();
    if groups_len > nums_len {
        return false;
    }
    let mut map = HashMap::new();
    for (idx, &val) in nums.iter().enumerate() {
        map.entry(val).or_insert_with(Vec::new).push(idx);
    }
    let mut idx = 0;
    for group in groups {
        let first_val = *group.first().unwrap();
        if !map.contains_key(&first_val) {
            return false;
        } else {
            let res = map.get(&first_val).unwrap().iter().find(|val_idx| {
                **val_idx >= idx && is_nums_contain_group_from_idx(&nums, **val_idx, &group)
            });
            if res.is_none() {
                return false;
            } else {
                idx = *res.unwrap() + group.len();
            }
        }
    }
    return true;
}

fn is_nums_contain_group_from_idx(nums: &Vec<i32>, idx: usize, group: &Vec<i32>) -> bool {
    for (i, &val) in group.iter().enumerate() {
        if idx + i >= nums.len() {
            return false;
        }
        if val == nums[idx + i] {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ans = can_choose(
            vec![vec![1, -1, -1], vec![3, -2, 0]],
            vec![1, -1, 0, 1, -1, -1, 3, -2, 0],
        );
        assert_eq!(ans, true);
    }

    #[test]
    fn test2() {
        let ans = can_choose(
            vec![vec![10, -2], vec![1, 2, 3, 4]],
            vec![1, 2, 3, 4, 10, -2],
        );
        assert_eq!(ans, false);
    }

    #[test]
    fn test3() {
        let ans = can_choose(
            vec![vec![1, 2, 3], vec![3, 4]],
            vec![7, 7, 1, 2, 3, 4, 7, 7],
        );
        assert_eq!(ans, false);
    }
}
