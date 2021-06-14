unsafe fn guess(num: i32) -> i32 {
    0
}

unsafe fn guessNumber(n: i32) -> i32 {
    let mut l = 0;
    let mut r = n;
    while l <= r {
        let mid = l + ((r - l) / 2);
        if  guess(mid) == 0 {
            return mid;
        } else if guess(mid) == 1 {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    l
}

