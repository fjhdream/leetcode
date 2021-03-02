struct NumMatrix {
    sum : Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        if m > 0 {
            let n = matrix[0].len();
            let mut sum = vec![vec![0;n+1]; m+1];
            for i in 0..m {
                for j in 0..gitn {
                    sum[i+1][j+1] = sum[i+1][j] + sum[i][j+1] - sum[i][j] + matrix[i][j];
                }
            }
            return NumMatrix { sum }
        }
        return NumMatrix { sum : Vec::new()}
       
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        return self.sum[row2+1][col2+1] - self.sum[row1][col2+1] - self.sum[row2+1][col1] + self.sum[row1][col1];
    }
}