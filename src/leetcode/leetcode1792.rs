use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord)]
struct Class {
    students: i64,
    seats: i64,
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let val1 = (other.seats + 1) * other.seats * (self.seats - self.students);
        let val2 = (self.seats + 1) * self.seats * (other.seats - other.students);
        if val1 == val2 {
            return Some(std::cmp::Ordering::Equal);
        }
        if val1 > val2 {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let mut priority_queue: BinaryHeap<Class> = std::collections::BinaryHeap::new();
    for class in &classes {
        priority_queue.push(Class {
            students: class[0] as i64,
            seats: class[1] as i64,
        });
    }
    for _ in 0..extra_students {
        let mut class = priority_queue.pop().unwrap();
        class.students += 1;
        class.seats += 1;
        priority_queue.push(class);
    }
    let mut sum = 0.0;
    for class in priority_queue {
        sum += class.students as f64 / class.seats as f64;
    }
    sum / classes.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1792() {
        assert_eq!(
            max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.78333
        );
        assert_eq!(
            max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
            0.53485
        );
    }
}
