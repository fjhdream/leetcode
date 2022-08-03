pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort();
    let mut diff = i32::MAX;
    let mut ans = vec![vec![]];
    for i in 0..arr.len()-1 {
        let tmp_diff = arr[i+1] - arr[i];
        if tmp_diff < diff {
            diff = tmp_diff;
            ans.clear();
            ans.push(vec![arr[i], arr[i+1]]);
        } else if tmp_diff == diff {
            ans.push(vec![arr[i], arr[i+1]]);
        }
    }
    return ans;
}