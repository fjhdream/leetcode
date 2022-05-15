pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut res = 0;
    let strs = strs.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let cols = strs[0].len();
    let rows = strs.len();
    for i in 0..cols {
        let mut is_ok = true;
        for j in 0..rows - 1 {
            if strs[j][i] > strs[j + 1][i] {
                is_ok = false;
                break;
            }
        }
        if !is_ok {
            res += 1;
        }
    }
    res
}