pub fn rob(numbs: Vec<i32>) -> i32 {
    let n = numbs.len();
    if n == 1 {
        return numbs[0];
    }
    if n == 2 {
        return numbs[0].max(numbs[1]);
    }
    get_rob_max(&numbs, 0, n - 2).max(get_rob_max(&numbs, 1, n - 1))
}

fn get_rob_max(numbs: &Vec<i32>, start: usize, end: usize) -> i32 {
    let mut first = numbs[start];
    let mut second = numbs[start].max(numbs[start + 1]);
    for i in start+2..=end {
        let tmp = second;
        second = second.max(first + numbs[i]);
        first = tmp;
    }
    second
}

#[test]
fn test_num() {
    let numbs = vec![1, 3, 1, 2, 100];
    let ans = rob(numbs);
    assert_eq!(ans, 103)
}