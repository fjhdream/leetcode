pub fn gray_code(n: i32) -> Vec<i32> {
    let mut res = vec![0];
    let mut head = 1;
    (0..n).for_each(|_| {
        (0..res.len()).rev().for_each(|j| {
            res.push(head + res[j]);
        });
        head <<= 1;
    });
    res
}

#[test]
fn test() {
    let n = 2;
    let ans = gray_code(n);
    assert_eq!(ans, vec![0, 1, 3, 2]);
}
