use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut map = HashMap::new();
    let count = nums.len() / 3;
    for num in nums {
        let val = map.entry(num).or_insert(0);
        *val += 1;
        if *val == count as i32 + 1{
            ans.push(num);
        }
    }
    ans
}

pub fn majority_element2(nums: Vec<i32>) -> Vec<i32> {
    let(mut candidate1, mut vote1) = (0, 0);
    let(mut candidate2, mut vote2) = (0, 0); 
    for &num in nums.iter() {
        if vote1 > 0 && candidate1 == num {
            vote1 += 1;
        } else if vote2 > 0 && candidate2 == num {
            vote2 += 1;
        } else if vote1 == 0 {
            candidate1 = num;
            vote1 = 1;
        } else if vote2 == 0 {
            candidate2 = num;
            vote2 = 1;
        } else {
            vote1 -= 1;
            vote2 -= 1;
        }
    }

    let (mut cnt1, mut cnt2) = (0, 0);
    for &num in nums.iter() {
        if num == candidate1 {
            cnt1 += 1;
        }
        if num == candidate2 {
            cnt2 += 1;
        }
    }
    let level = nums.len() / 3;
    let mut ans = Vec::new();
    if vote1 > 0 && cnt1 > level {
        ans.push(candidate1);
    }
    if vote2 > 0 && cnt2 > level {
        ans.push(candidate2);
    }

    ans
}

#[test]
fn test() {
    let ans = majority_element2(vec![1]);
    assert_eq!(ans, vec![1]);
}

#[test]
fn test2() {
    let ans = majority_element2(vec![1,3,3,4]);
    assert_eq!(ans, vec![3]);
}