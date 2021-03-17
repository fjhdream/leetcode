pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut num = 1;
    let n = n as usize;
    let mut result = vec![vec![0;n];n];
    let (mut top, mut bottom, mut left, mut right) = (0, n-1, 0, n-1);
    while top <= bottom && left <= right{
        for i in left..=right  {
            result[top][i] = num;
            num += 1;
        }
        for i in top+1..=bottom {
            result[i][right] = num;
            num += 1;
        }
        if top < bottom && left < right{
            for i in (left..right).rev() {
                result[bottom][i] = num;
                num += 1;
            }
            for i in (top+1..bottom).rev() {
                result[i][left] = num;
                num += 1;
            }
        }
        top += 1;
        if bottom == 0 {
            break;
        }
        bottom -= 1;
        left += 1;
        right -= 1;
    }
    result
}

#[test]
pub fn test_example() {
    let ans = generate_matrix(1);
    println!("{:?}", ans);
}