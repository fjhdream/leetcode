pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut remainders = vec![0_i64; 60];
    let mut count = 0;
    for t in time {
        let remainder = t % 60;
        remainders[remainder as usize] += 1;
    }
    for i in 1..30 {
        count += remainders[i] * remainders[60 - i];
    }
    count += remainders[0] * (remainders[0] - 1) / 2;
    count += remainders[30] * (remainders[30] - 1) / 2;
    count as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::num_pairs_divisible_by60(vec![30, 20, 150, 100, 40]),
            3
        );
    }
}
