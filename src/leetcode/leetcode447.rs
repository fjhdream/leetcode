pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut ans = 0;
    for point in &points {
        let mut map = HashMap::new();
        for other_point in &points { 
            let dist = dist_square(point, other_point);
            let entry = map.entry(dist).or_insert(0);
            *entry += 1;
        }
        for (_, value) in map.iter() {
            ans += value * (value - 1);
        }
    }
    ans
}

fn dist_square(point_x: &Vec<i32>, point_y: &Vec<i32>) -> i32 {
    return (point_x[0] - point_y[0]).pow(2) + (point_x[1] - point_y[1]).pow(2);
}

#[test]
fn test_example() {
    let points = vec![vec![0,0], vec![1,0], vec![2,0]];
    let ans = number_of_boomerangs(points);
    assert_eq!(ans, 2);
}

#[test]
fn test_example2() {
    let points = vec![vec![1,1], vec![2,2], vec![3,3]];
    let ans = number_of_boomerangs(points);
    assert_eq!(ans, 2);
}

#[test]
fn test_example3() {
    let points = vec![vec![1,1]];
    let ans = number_of_boomerangs(points);
    assert_eq!(ans, 0);
}