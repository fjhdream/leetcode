pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let courses = num_courses as usize;
    let mut grid = vec![vec![false; courses]; courses];
    for prerequist in prerequisites {
        let (pre, post) = (prerequist[0] as usize, prerequist[1] as usize);
        grid[pre][post] = true;
    }
    for k in 0..courses {
        for i in 0..courses {
            for j in 0..courses {
                if grid[i][k] && grid[k][j] {
                    grid[i][j] = true;
                }
            }
        }
    }

    let mut res = vec![];
    for query in queries {
        let(x, y) = (query[0] as usize, query[1] as usize);
        res.push(grid[x][y]);
    }
    res
}

#[test]
fn test() {
    let ans = check_if_prerequisite(2, vec![vec![1,0]], vec![vec![0,1], vec![1,0]]);
    assert_eq!(ans, vec![false, true]);
}

#[test]
fn test2() {
    let ans = check_if_prerequisite(2, vec![], vec![vec![0,1], vec![1,0]]);
    assert_eq!(ans, vec![false, false]);
}

#[test]
fn test5() {
    let ans = check_if_prerequisite(5, 
        vec![vec![0,1],vec![1,2],vec![2,3],vec![3,4]], 
        vec![ vec![0,4], vec![4,0], vec![1,3], vec![3,0]]);
    assert_eq!(ans, vec![true,false,true,false]);
}