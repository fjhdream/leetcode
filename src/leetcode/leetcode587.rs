pub fn outer_trees(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // 按照x升序排序, y升序排序
    trees.sort();

    let n = trees.len();
    // 栈顶元素下标
    let mut tp = 0;
    let mut stack = vec![0; n + 10];
    let mut visited = vec![false; n + 10];
    // 不标记起点
    stack[tp + 1] = 0;
    tp += 1;

    for i in 1..n {
        let c = &trees[i];
        while tp >= 2 {
            let (a, b) = (&trees[stack[tp - 1]], &trees[stack[tp]]);
            if get_area(a, b, c) < 0f64 {
                visited[stack[tp]] = false;
                tp -= 1;
            } else {
                break;
            }
        }
        stack[tp + 1] = i;
        tp += 1;
        visited[i] = true;
    }

    let size = tp;
    for i in (0..n).rev() {
        if visited[i] {
            continue;
        }
        let c = &trees[i];
        while tp > size {
            let(a, b) = (&trees[stack[tp - 1]], &trees[stack[tp]]);
            if get_area(a, b, c) < 0f64 {
                //非必需
                visited[stack[tp]] = false;
                tp -= 1;
            } else {
                break;
            }
        }
        stack[tp + 1] = i;
        tp += 1;
        //非必需
        visited[i] = true;
    }

    let mut res = vec![];
    for i in 1..tp {
        res.push(trees[stack[i]].clone());
    }
    res
}

fn get_area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> f64 {
    cross(&subtraction(b, a),  &subtraction(c, a))
}

fn cross(a: &Vec<i32>, b: &Vec<i32>) -> f64 {
    return (a[0] * b[1] - a[1] * b[0]).into();
}

fn subtraction(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    return vec![a[0] - b[0], a[1] - b[1]];
}

#[test]
fn test() {
    let ans = outer_trees(vec![vec![1,1], vec![2,2], vec![2,0], vec![2,4], vec![3,3], vec![4,2]]);
    print!("{:?}", ans)
}


