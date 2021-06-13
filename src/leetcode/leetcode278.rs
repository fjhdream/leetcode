struct Solution278 {}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl Solution278 {
    fn isBadVersion(&self, versions:i32)-> bool {
        return false;
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
		if n == 0 {
            return 0;
        }
        let mut left = 1;
        let mut right = n;
        while right > left {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}