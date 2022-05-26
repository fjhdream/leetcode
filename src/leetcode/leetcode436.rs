pub fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();
    let mut res = vec![-1; n];
    let mut start_intervals = vec![vec![0;2]; n];
    for i in 0..n {
        start_intervals[i][0] = intervals[i][0];
        start_intervals[i][1] = i as i32;
    }

    start_intervals.sort_by(|i1, i2| ->  std::cmp::Ordering {
        i1[0].cmp(&i2[0])
    });
    for i in 0..n {
        let(mut left, mut right) = (0 as i32, n as i32 - 1);
        let mut target = -1;
        while left <= right {
            let mid = (left + right) / 2;
            if start_intervals[mid as usize][0] >= intervals[i][1] {
                target = start_intervals[mid as usize][1];
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        res[i] = target;
    }
    res
}

#[test]
fn test() {
    let ans = find_right_interval(vec![vec![1,1], vec![3,4]]);
    println!("{:?}", ans);
}