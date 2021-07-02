pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
    let mut ans = 0;
    costs.sort();
    for cost in costs.iter() {
        if coins >= *cost {
            coins -= *cost;
            ans += 1;
        } else {
            break;
        }
    }
    ans
}