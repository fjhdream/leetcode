pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
    let n = encoded.len();
    let mut ans = vec![0; n+1];
    let mut all = 0;
    for i in 1..=n+1 {
        all ^= i as i32;
    }
    let mut odd: i32 = 0;
    for i in (1..n).step_by(2)  {
        odd ^= encoded[i];
    }
    ans[0] = all ^ odd;
    for i in 0..n {
        ans[i+1] = ans[i] ^ encoded[i];
    }
    ans
}


#[test]
pub fn test() {
    let encoded = vec![3,1];
    let ans = decode(encoded);
    println!("{:?}", ans);
}