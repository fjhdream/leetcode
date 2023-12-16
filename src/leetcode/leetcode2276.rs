struct CountIntervals {
    left: Option<Box<CountIntervals>>,
    right: Option<Box<CountIntervals>>,
    l: i32,
    r: i32,
    cnt: i32,
}

impl CountIntervals {
    fn create(l: i32, r: i32) -> Self {
        CountIntervals {
            left: None,
            right: None,
            l,
            r,
            cnt: 0,
        }
    }

    fn new() -> Self {
        Self::create(1, 1_000_000_000)
    }

    fn add(&mut self, l_interval: i32, r_interval: i32) {
        if self.cnt == self.r - self.l + 1 {
            return;
        }
        if l_interval <= self.l && r_interval >= self.r {
            self.cnt = self.r - self.l + 1;
            return;
        }
        let mid = (self.l + self.r) / 2;
        if l_interval <= mid {
            if self.left.is_none() {
                self.left = Some(Box::new(CountIntervals::create(self.l, mid)));
            }
            self.left.as_mut().unwrap().add(l_interval, r_interval);
        }
        if mid < r_interval {
            if self.right.is_none() {
                self.right = Some(Box::new(CountIntervals::create(mid + 1, self.r)));
            }
            self.right.as_mut().unwrap().add(l_interval, r_interval);
        }
        self.cnt = self.left.as_ref().map_or(0, |left| left.cnt)
            + self.right.as_ref().map_or(0, |right| right.cnt);
    }

    fn count(&self) -> i32 {
        self.cnt
    }
}
