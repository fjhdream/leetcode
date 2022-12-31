fn prime_factors(n: i64) -> String {
    let mut n = n;
    let mut factors = Vec::new();

    let mut factor = 2;
    while n > 1 {
        let mut count = 0;
        while n % factor == 0 {
            count += 1;
            n /= factor;
        }
        if count > 0 {
            factors.push((factor, count));
        }
        factor += 1;
    }
    factors
        .into_iter()
        .map(|(factor, count)| match count {
            1 => format!("({})", factor),
            _ => format!("({}**{})", factor, count),
        })
        .collect()
}

fn calculate_primes(n: i64) -> Vec<i64> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n as usize + 1 {
        if is_prime[i] {
            primes.push(i as i64);
            let mut j = i * i;
            while j < n as usize + 1 {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
