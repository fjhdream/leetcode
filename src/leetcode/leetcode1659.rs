
pub struct Solution {
    dp: Box<[[[[i32;7];7];243];25]>,
    mask_span: Box<[[i32;5];243]>,
    truncate: Box<[[usize;3];243]>
}

impl Solution {
    fn new() -> Solution {
        Solution {
            dp: Box::new([[[[0;7];7];243];25]),
            mask_span: Box::new([[0;5];243]),
            truncate: Box::new([[0;3];243])
        }
    } 
    #[allow(dead_code)]
    pub fn get_max_grid_happiness(&mut self, m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
        let max_mask = 3_i32.pow(n as u32);
        let highest_mask = max_mask / 3;
        for mask in 0..max_mask {
            let mut mask_tmp = mask;
            for i in 0..n {
                self.mask_span[mask as usize][(n-i-1) as usize] = mask_tmp % 3;
                mask_tmp /= 3;
            }
            self.truncate[mask as usize][0] = (mask % highest_mask * 3) as usize;
            self.truncate[mask as usize][1] = (mask % highest_mask * 3 + 1) as usize;
            self.truncate[mask as usize][2] = (mask % highest_mask * 3 + 2) as usize;
        }
        self.dfs(0, 0, introverts_count, extroverts_count, m, n)
    }

    fn dfs(&mut self, pos: usize, border_line: usize, nx: i32, wx: i32, m: i32, n:i32) -> i32 {
        if pos as i32 == m * n || nx + wx == 0 {
            return 0;
        }
        if self.dp[pos][border_line][nx as usize][wx as usize] != 0 {
            return self.dp[pos][border_line][nx as usize][wx as usize];
        }
        let y = pos % n as usize;
        let mut best = self.dfs(pos + 1, self.truncate[border_line][0], nx, wx, m, n);
        if nx > 0  {
            best = best.max(
                120 
                + self.calculate(1, self.mask_span[border_line][0]) 
                + (if y == 0 {0} else { self.calculate(1, self.mask_span[border_line][ (n-1) as usize])}) 
                + self.dfs(pos + 1, self.truncate[border_line][1], nx -1, wx, m, n)
            );
        }
        if wx > 0 {
            best = best.max(
                40
                + self.calculate(2, self. mask_span[border_line][0])
                + (if y == 0 {0} else { self.calculate(2, self.mask_span[border_line][(n-1) as usize])})
                + self.dfs(pos + 1, self.truncate[border_line][2], nx, wx-1, m, n)
            );
        }
        self.dp[pos][border_line][nx as usize][wx as usize] = best;
        best
    }
    
    fn calculate(&mut self, x: i32, y: i32) -> i32 {
        if x==0 || y==0 {
            return 0;
        }
        if x == 1 && y == 1 {
            return -60;
        }
        if x==2 && y==2 {
            return 40;
        }
        -10
    }
 
}


#[test]
fn test() {
    let m = 2;
    let n = 3;
    let introverts_count = 1;
    let extroverts_count = 2;
    let mut solution = Solution::new();
    let ans = solution.get_max_grid_happiness( m, n, introverts_count, extroverts_count);
    println!("{}", ans);
    assert_eq!(240, ans);
}