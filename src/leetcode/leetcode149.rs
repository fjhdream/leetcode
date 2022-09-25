use std::collections::HashMap;
pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    if n <= 2 {
        return n as i32;
    }
    let mut ans = i32::MIN;
    for i in 0..n {
        let mut map = HashMap::new();
        for j in i+1..n {
            let x = points[i][0] - points[j][0];
            let y = points[i][1] - points[j][1];
            let slope = get_slope(x, y);
            if map.contains_key(&slope) {
                map.insert(slope, map.get(&slope).unwrap() + 1);
            } else {
                map.insert(slope, 2);
            }
        }
        if let Some(value) = map.values().max() {
            ans = ans.max(*value);
        }
    }
    ans
}

fn get_slope(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    if x == 0 {
        y = 1;
    } else if y == 0 {
        x = 1;
    } else {
        if y < 0 {
            x = -x;
            y = -y;
        }
        let gcb_xy = gcb(x, y);
        x /= gcb_xy;
        y /= gcb_xy;
    }
    let key = y + x * 200001;

    return key;
}

fn gcb(a: i32, b: i32)-> i32 {
    if b == 0 {
    return a;
    }
    return gcb(b, a %b );
}

#[test]
fn test_example() {
    let _points = vec![vec![1,1], vec![3,2], vec![5,3], vec![4,1], vec![2,3]];
    let points = vec![vec![1,1], vec![2,2], vec![3,3]];
    let ans = max_points(points);
    assert_eq!(ans, 3);
}