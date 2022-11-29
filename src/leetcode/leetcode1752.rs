pub fn check(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    let mut rev_cnt = 0;
    let mut last = nums[0];
    for i in 1..nums.len() {
        if nums[i] >= last {
            last = nums[i];
        } else {
            rev_cnt += 1;
            if rev_cnt > 1 {
                return false;
            }
            last = nums[i];
        }
    }
    return if rev_cnt == 0 {
        true
    } else {
        nums.last().unwrap() <= nums.first().unwrap()
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test() {
        assert!(check(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    pub fn test2() {
        assert!(!check(vec![2, 1, 3, 4]));
    }

    #[test]
    pub fn test3() {
        assert!(check(vec![1, 2, 3]));
    }

    #[test]
    pub fn test4() {
        assert!(check(vec![1, 1, 1]));
    }

    #[test]
    pub fn test5() {
        assert!(check(vec![1, 2, 1]));
    }

    #[test]
    pub fn test6() {
        assert!(check(vec![1, 1, 1, 2]));
    }

    #[test]
    pub fn test7() {
        assert!(check(vec![3, 1, 1, 1, 2, 2]));
    }

    #[test]
    pub fn test8() {
        assert!(check(vec![6, 10, 6]));
    }
}
