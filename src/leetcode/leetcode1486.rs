pub fn xor_operation(n: i32, start: i32) -> i32 {
    let s = start >> 1;
    let e = start & n & 1;
    (sum_xor(s - 1) ^ sum_xor(s + n - 1)) << 1 | e
}
fn sum_xor(x: i32) -> i32 {
    if x % 4 == 0 {
        return x;
    }
    if x % 4 == 1 {
        return 1;
    }
    if x % 4 == 2 {
        return x + 1;
    }
    0
}