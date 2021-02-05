pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let diff_vec:Vec<i32> = 
        s.chars()
        .zip(t.chars())
        .map(|(i, j)| (i as i32 - j as i32).abs())
        .collect();
    let (mut window_sum, mut slow_idx) = (0i32, 0);
    for fast_idx in 0..diff_vec.len() {
        window_sum += diff_vec[fast_idx];
        if window_sum > max_cost {
            window_sum -= diff_vec[slow_idx];
            slow_idx += 1;
        }
    }
    (diff_vec.len() - slow_idx) as i32
}