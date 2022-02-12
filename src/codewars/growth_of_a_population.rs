fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut ans = 0;
    let mut person = p0;
    while person < p {
        person += (person as f64 * percent * 0.01f64 + aug as f64) as i32;
        ans += 1;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans = nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
        dotest(41373, 0.46, 206, 1045081, 554);
    }
}
