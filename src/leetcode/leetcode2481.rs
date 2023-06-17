pub fn number_of_cuts(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    if n % 2 == 0 {
        return n / 2;
    }
    n
}
