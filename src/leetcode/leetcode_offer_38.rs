#[allow(dead_code)]
pub fn permutation(mut s: String) -> Vec<String> {
    let n = s.len();
    let mut res = Vec::new();
    let mut t = String::with_capacity(n);
    let mut visited = vec![false; n];
    let s = unsafe { s.as_bytes_mut() };
    s.sort();
    fn backtrace(pos: usize, res: &mut Vec<String>, 
        t: &mut String, visited: &mut Vec<bool>, n : usize, s: &[u8]) {
        if pos == n {
            res.push(t.clone());
            return;
        }
        for i in 0..n {
            if visited[i] || (i > 0 && s[i-1] == s[i] && !visited[i-1]) {
                continue;                
            }
            t.push(s[i] as char);
            visited[i] = true;
            backtrace(pos+1, res, t, visited, n, s);
            t.pop();
            visited[i] = false;
        }

    }
    backtrace(0, &mut res, &mut t, &mut visited, n, s);

    res
}

#[test]
fn test_example() {
    let s = String::from("aab");
    let ans = permutation(s);
    println!("{:?}", ans);
}