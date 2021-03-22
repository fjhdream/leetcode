pub fn hammingWeight (n: u32) -> i32 {
    let mut weight = 0;
    let mut num = n;
    while num > 0 {
        num = num & (num - 1);
        weight += 1;
    }
    weight
}