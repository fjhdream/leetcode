pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let n = digits.len();
    for i in (0..n).rev() {
        if digits[i] != 9 {
            digits[i] += 1;
            for j in i + 1..n {
                digits[j] = 0;
            }
            return digits;
        }
    }

    let mut ans = vec![0; n + 1];
    ans[0] = 1;
    return ans;
}
