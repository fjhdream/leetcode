pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let n = card_points.len() as i32;
    if n < k {
        return -1;
    }
    if n == k {
        return card_points.iter().sum();
    }

    let sum = card_points.iter().sum::<i32>();
    let window_size = (n - k) as usize;
    let mut ans = i32::MIN;
    let mut window_sum = 0;
    for i in 0..window_size {
        window_sum += card_points[i];
    }
    ans = ans.max(sum - window_sum);
    for i in window_size..n as usize {
        window_sum -= card_points[i - window_size];
        window_sum += card_points[i];
        ans = ans.max(sum - window_sum);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;
        let ans = max_score(card_points, k);
        assert_eq!(ans, 12);
    }
}
