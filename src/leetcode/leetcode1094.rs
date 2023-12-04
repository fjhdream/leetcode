pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut diff = vec![0; 1010];
    for trip in trips {
        let passages = trip[0];
        let add = trip[1];
        let sub = trip[2];
        diff[add as usize + 1] += passages;
        diff[sub as usize + 1] -= passages;
    }

    let mut current_passages = 0;
    for i in 1..=1000 {
        current_passages += diff[i];
        if current_passages > capacity {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_pooling() {
        // Test case 1
        let trips1 = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity1 = 4;
        assert_eq!(car_pooling(trips1, capacity1), false);

        // Test case 2
        let trips2 = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity2 = 5;
        assert_eq!(car_pooling(trips2, capacity2), true);

        // Test case 3
        let trips3 = vec![vec![2, 1, 5], vec![3, 3, 7]];
        let capacity3 = 10;
        assert_eq!(car_pooling(trips3, capacity3), true);
    }
}
