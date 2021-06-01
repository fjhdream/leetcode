pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = candies_count.len();
    let mut pre_sum = vec![0 as i64; n];
    pre_sum[0] = candies_count[0] as i64;
    for i in 1..n {
        pre_sum[i] = pre_sum[i-1] + candies_count[i] as i64;
    }

    let m = queries.len();
    let mut ans = vec![true; m];
    for (i, query) in queries.iter().enumerate() {
        let (favorite_type, favorite_day, daily_cap) = (query[0], query[1], query[2]);
        let min_sum = if favorite_type == 0 { 1 } else {pre_sum[favorite_type as usize-1] as i64 + 1};
        let max_sum = pre_sum[favorite_type as usize] as i64;
        let min_eat = favorite_day as i64 + 1 ;
        let max_eat = (favorite_day + 1) as i64 * daily_cap as i64;
        ans[i] = !(min_eat > max_sum || max_eat < min_sum)
    }
    ans
}

#[test]
pub fn test() {
    let candies_count = vec![7,4,5,3,8];
    let queries = vec![vec![0,2,2], vec![4,2,4], vec![2,13,1000000000]];
    let ans = can_eat(candies_count, queries);
    println!("{:?}", ans);
}