#[warn(dead_code)]
pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let mut max_len = i32::MIN;
    let mut ans = 0;
    for rectangle in rectangles {
        let len = rectangle[0].min(rectangle[1]);
        if max_len < len {
            max_len = len;
            ans = 1;
        } else if len == max_len {
            ans += 1;
        }
    }
    ans
}

#[test]
fn test() {
    let rectangles = vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]];
    let ans = count_good_rectangles(rectangles);
    assert_eq!(ans, 3);
}

#[test]
fn test2() {
    let rectangles = vec![vec![5, 8]];
    let ans = count_good_rectangles(rectangles);
    assert_eq!(ans, 1);
}
