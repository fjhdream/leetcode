pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
    let water = -1;
    let mut cur_capacity = capacity;
    let mut res = 0;
    let mut cur = -1;
    for (idx, &plant) in plants.iter().enumerate() {
        if cur_capacity < plant {
            res += cur - water;
            cur_capacity = capacity;
            cur = water;
        }
        res += idx as i32 - cur;
        cur = idx as i32;
        cur_capacity = cur_capacity - plant;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let plants = vec![7, 7, 7, 7, 7, 7, 7];
        let capacity = 8;
        let ans = watering_plants(plants, capacity);
        assert_eq!(ans, 49);
    }
}
