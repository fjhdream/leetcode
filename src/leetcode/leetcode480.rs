use std::collections::VecDeque;
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    median_sliding_window_process(&nums, k as usize)
}

fn median_sliding_window_process(num: &[i32], k: usize) -> Vec<f64> {
    let (mut window, mut sorted_window) = (VecDeque::with_capacity(k), Vec::with_capacity(k));
    let mut medians = Vec::with_capacity(num.len().max(k) - k + 1);
    for &n in num {
        window.push_back(n);
        sorted_window.insert(sorted_window.binary_search(&n).unwrap_or_else(|i| i), n);
        if sorted_window.len() < k {
            continue;
        }
        if sorted_window.len() > k {
            sorted_window.remove(sorted_window.binary_search(&window.pop_front().unwrap()).unwrap());
        }
        medians.push(if k%2 == 1 {
            sorted_window[k/2] as f64
        }else {
            (sorted_window[k / 2] as f64 + sorted_window[k / 2 - 1] as f64) / 2.0
        });
    }
    medians
}
