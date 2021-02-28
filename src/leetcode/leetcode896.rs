pub fn is_monotonic(a: Vec<i32>) -> bool {
    let (mut inc, mut dec) = (true, true);
    for i in 1..a.len() {
        if a[i] > a[i-1] {
            dec = false;
        }
        if a[i] < a[i-1] {
            inc = false;
        }
    }
    inc | dec
}