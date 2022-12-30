use std::{
    cmp::Ordering,
    collections::{BTreeSet, BinaryHeap},
};

#[derive(Ord, Eq)]
struct Pair(i32, i32);

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd for Pair {
    // 两个座位距离相差为1以内的, 则视为他与离他最近的人之间的距离是一样的, 中间间隔数(1,2) (3,4) 是一样的 中间间隔数(2,3) 则可以直接比大小
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let my_scope = self.1 - self.0;
        let other_scope = other.1 - other.0;
        if (my_scope - other_scope).abs() > 1
            || ((my_scope - other_scope).abs() == 1 && my_scope.min(other_scope) & 1 == 1)
        {
            if my_scope < other_scope {
                return Some(Ordering::Less);
            } else {
                return Some(Ordering::Greater);
            }
        } else {
            if self.0 > other.0 {
                Some(Ordering::Less)
            } else if self.0 < other.0 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

struct ExamRoom {
    is_exist: bool,
    seated_set: BTreeSet<i32>,
    priority_queue: BinaryHeap<Pair>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    fn new(n: i32) -> Self {
        ExamRoom {
            is_exist: false,
            seated_set: BTreeSet::new(),
            priority_queue: BinaryHeap::new(),
            n,
        }
    }

    fn seat(&mut self) -> i32 {
        if !self.is_exist {
            self.seated_set.insert(0);
            self.is_exist = true;
            return 0;
        }

        let left = *self.seated_set.iter().min().unwrap();
        let right = self.n - 1 - *self.seated_set.iter().max().unwrap();
        while !self.priority_queue.is_empty() {
            let seat_pair = self.priority_queue.pop().unwrap();
            // 两端座位需要有人坐, 且是有效的座位区间
            if self.seated_set.contains(&seat_pair.0)
                && self.seated_set.contains(&seat_pair.1)
                && self
                    .seated_set
                    .iter()
                    .zip(self.seated_set.iter().clone().skip(1))
                    .into_iter()
                    .any(|(&a, &b)| a == seat_pair.0 && b == seat_pair.1)
            {
                let dist = seat_pair.1 - seat_pair.0;
                if dist / 2 < right || dist / 2 <= left {
                    self.priority_queue.push(seat_pair);
                    break;
                }
                let mid = seat_pair.0 + (seat_pair.1 - seat_pair.0) / 2;
                self.seated_set.insert(mid);
                if mid - seat_pair.0 > 1 {
                    self.priority_queue.push(Pair(seat_pair.0, mid));
                }
                if seat_pair.1 - mid > 1 {
                    self.priority_queue.push(Pair(mid, seat_pair.1));
                }
                return mid;
            }
        }

        // 两端没有人坐下且最优时
        if right > left {
            self.priority_queue
                .push(Pair(*self.seated_set.iter().max().unwrap(), self.n - 1));
            self.seated_set.insert(self.n - 1);
            return self.n - 1;
        } else {
            self.priority_queue
                .push(Pair(0, *self.seated_set.iter().min().unwrap()));
            self.seated_set.insert(0);
            return 0;
        }
    }

    fn leave(&mut self, p: i32) {
        self.seated_set.remove(&p);
        if self.seated_set.is_empty() {
            self.is_exist = false;
            return;
        } else {
            let mut pre: Option<i32> = Option::None;
            let mut post: Option<i32> = Option::None;
            for seat in self.seated_set.iter() {
                if *seat < p {
                    pre = Some(*seat);
                } else if *seat > p {
                    post = Some(*seat);
                    break;
                }
            }
            if pre == Option::None || post == Option::None {
                return;
            }
            self.priority_queue.push(Pair(pre.unwrap(), post.unwrap()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exam_room() {
        let mut exam_room = ExamRoom::new(10);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 9);
        assert_eq!(exam_room.seat(), 4);
        assert_eq!(exam_room.seat(), 2);
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 5);
    }

    #[test]
    fn test_exam_room2() {
        let mut exam_room = ExamRoom::new(4);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 3);
        assert_eq!(exam_room.seat(), 1);
        assert_eq!(exam_room.seat(), 2);
        exam_room.leave(1);
        assert_eq!(exam_room.seat(), 1);
    }

    #[test]
    fn test_exam_room3() {
        let mut exam_room = ExamRoom::new(10);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 9);
        assert_eq!(exam_room.seat(), 4);
        exam_room.leave(0);
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 4);
        assert_eq!(exam_room.seat(), 2);
        assert_eq!(exam_room.seat(), 6);
        assert_eq!(exam_room.seat(), 1);
        assert_eq!(exam_room.seat(), 3);
        assert_eq!(exam_room.seat(), 5);
        assert_eq!(exam_room.seat(), 7);
        assert_eq!(exam_room.seat(), 8);
        exam_room.leave(0);
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 4);
        exam_room.leave(7);
        assert_eq!(exam_room.seat(), 7);
    }

    #[test]
    fn test_exam_room4() {
        let mut exam_room = ExamRoom::new(7);
        assert_eq!(exam_room.seat(), 0);
        assert_eq!(exam_room.seat(), 6);
        assert_eq!(exam_room.seat(), 3);
        assert_eq!(exam_room.seat(), 1);
        exam_room.leave(1);
        assert_eq!(exam_room.seat(), 1);
        assert_eq!(exam_room.seat(), 2);
        assert_eq!(exam_room.seat(), 4);
        assert_eq!(exam_room.seat(), 5);
    }
}
