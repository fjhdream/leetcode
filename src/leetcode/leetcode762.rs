pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut count = 0;
    for i in left..=right {
        count += count_prime_bits(i);
    }
    count    
}

fn count_prime_bits(i: i32) -> i32 {
    let mut count = i.count_ones() as i32;
    if is_prime(count) {
        return 1;
    } else {
        return 0;
    }

}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
        
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[test]
fn test() {
    let ans = count_prime_set_bits(6, 10);
    assert_eq!(ans, 4);
}