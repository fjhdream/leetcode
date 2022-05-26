pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut n1, mut y1) = (0, 1);
    let n = nums1.len();
    for i in 1..n {
        let (mut n2, mut y2) = (i32::MAX, i32::MAX); 
        if nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i] {
            n2 = n2.min(n1);
            y2 = y2.min(y1 + 1);
        }
        // i-1 列 和 i列有一列交换
        if nums1[i - 1] < nums2[i] && nums2[i - 1] < nums1[i] {
            n2 = n2.min(y1);
            y2 = y2.min(n1 + 1);
        }
        n1 = n2;
        y1 = y2;
    }
    n1.min(y1)
}

#[test]
fn test() {
    let ans = min_swap(vec![1,3,5,4], vec![1,2,3,7]);
    assert_eq!(ans, 1);
}