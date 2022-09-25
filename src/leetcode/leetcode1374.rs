pub fn generate_the_string(n: i32) -> String {
    let a = 'a';
    let b = 'b';
    let mut ans = String::new();
    if n & 1 == 1 {
        for _ in 0..n {
            ans.push(a);
        }
    } else {
        for _ in 0..n - 1 {
            ans.push(a);
        }
        ans.push(b);
    }
    return ans;
}
