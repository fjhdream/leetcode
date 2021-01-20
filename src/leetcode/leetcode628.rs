use std::i32::{MIN, MAX};
use std::cmp::max;
pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;
    let mut max3 = i32::MIN;

    for x in nums {
        if x < min1 {
            min2 = min1;
            min1 = x;
        } else if x < min2 {
            min2 = x;
        }
        if x > max1 {
            max3 = max2;
            max2 = max1;
            max1 = x;
        } else if x > max2 {
            max3 = max2;
            max2 = x;
        } else if x > max3 {
            max3 = x;
        }
    }
    max(min1*min2*max1, max1*max2*max3)
}
