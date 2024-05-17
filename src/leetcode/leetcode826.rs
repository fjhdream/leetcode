pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();
    jobs.sort_by_key(|&(d, _)| d);
    let mut worker = worker;
    worker.sort();

    let mut res = 0;
    let mut i = 0;
    let mut best = 0;
    let n = jobs.len();

    // Iterate over the sorted worker array
    for &w in &worker {
        // Find the best profit for the current worker capability
        while i < n && w >= jobs[i].0 {
            best = best.max(jobs[i].1);
            i += 1;
        }
        res += best;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let difficulty = vec![2, 4, 6, 8, 10];
        let profit = vec![10, 20, 30, 40, 50];
        let worker = vec![4, 5, 6, 7];
        assert_eq!(max_profit_assignment(difficulty, profit, worker), 100);
    }
}
