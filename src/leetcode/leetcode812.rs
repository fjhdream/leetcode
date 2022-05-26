pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut max = 0.0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let area = three_points_area(&points[i], &points[j], &points[k]);
                if area > max {
                    max = area;
                }
            }
        }
    }
    max
}

pub fn largest_triangle_area2(points: Vec<Vec<i32>>) -> f64 {
    let hulls = get_converx_hull(points);
    let n = hulls.len();
    let mut ret = 0.0f64;
    for i in 0..n {
        for j in i + 1..n - 1 {
            let mut k = j + 1;
            while k + 1 < n {
                let cur_area = three_points_area(&hulls[i], &hulls[j], &hulls[k]);
                let next_area = three_points_area(&hulls[i], &hulls[j], &hulls[k + 1]);
                if cur_area >= next_area {
                    break;
                }
                k += 1;
            }
            let area = three_points_area(&hulls[i], &hulls[j], &hulls[k]);
            ret = ret.max(area);
        }
    }
    ret
}

fn get_converx_hull(mut points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = points.len();
    if n < 4 {
        return points;
    }
    points.sort();
    let mut hull = Vec::new();
    //求凸包下班部分
    for i in 0..n {
        while hull.len() > 1 && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], &points[i]) <= 0
        {
            hull.pop();
        }
        hull.push(points[i].clone());
    }
    let m = hull.len();
    for i in (0..n - 1).rev() {
        while hull.len() > m && cross(&hull[hull.len() - 2], &hull[hull.len() - 1], &points[i]) <= 0
        {
            hull.pop();
        }
        hull.push(points[i].clone());
    }
    //  去掉最后一个点 points[0]
    hull.pop();
    return hull;
}

fn three_points_area(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> f64 {
    let (a, b) = (p1[0] as f64, p1[1] as f64);
    let (c, d) = (p2[0] as f64, p2[1] as f64);
    let (e, f) = (p3[0] as f64, p3[1] as f64);
    let s = (a * d + c * f + b * e - d * e - c * b - f * a) / 2.0;
    return s.abs();
}

fn cross(p: &Vec<i32>, q: &Vec<i32>, r: &Vec<i32>) -> i32 {
    return (q[0] - p[0]) * (r[1] - q[1]) - (q[1] - p[1]) * (r[0] - q[0]);
}

#[test]
fn test() {
    let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
    assert_eq!(2.0, largest_triangle_area2(points));
}
