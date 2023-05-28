use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Ord)]
struct V {
    v: i32,
    i: usize,
    j: usize,
}

impl PartialOrd for V {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.v.cmp(&other.v) != std::cmp::Ordering::Equal {
            return Some(self.v.cmp(&other.v).reverse());
        } else if self.i.cmp(&other.i) != std::cmp::Ordering::Equal {
            return Some(self.i.cmp(&other.i).reverse());
        } else {
            return Some(self.j.cmp(&other.j).reverse());
        }
    }
}

pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut pre = vec![0];
    for row in mat {
        pre = calculate_heap(&pre, &row, k);
    }
    pre[k as usize - 1]
}

fn calculate(pre: &[i32], row: &[i32], k: i32) -> Vec<i32> {
    let mut result = vec![];
    for &i in pre {
        for &j in row {
            result.push(i + j);
        }
    }
    result.sort_unstable();
    result.truncate(k as usize);
    result
}

fn calculate_heap(nums1: &Vec<i32>, nums2: &Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums1.len();
    let m = nums2.len();
    let sz = (k as usize).min(n * m);
    let mut result = vec![0; sz];
    let mut idx = 0_usize;
    let mut min_heap = BinaryHeap::new();
    min_heap.push(V{v: nums1[0] + nums2[0], i: 0, j: 0});
    while !min_heap.is_empty() && idx < k as usize {
        let V { v, i, j } = min_heap.pop().unwrap();
        result[idx] = v;
        idx += 1;
        if j == 0 && i + 1 < n {
            min_heap.push(V{ v: nums1[i + 1] + nums2[0], i: i + 1, j: 0_usize});
        }
        if j + 1 < m {
            min_heap.push(V{v: nums1[i] + nums2[j + 1], i: i, j: j + 1});
        }
    } 
    result
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let ans = kth_smallest(vec![vec![1,3,11], vec![2,4,6]], 5);
        assert_eq!(ans, 7);
    }


    #[test]
    fn test2() {
        let ans = kth_smallest(vec![vec![1,3,11], vec![2,4,6]], 9);
        assert_eq!(ans, 17);
    }

    #[test]
    fn test3() {
        let ans = kth_smallest(vec![vec![1,10,10], vec![1,4,5], vec![2,3,6]], 7);
        assert_eq!(ans, 9);
    }

    #[test]
    fn test4() {
        let ans = kth_smallest(vec![vec![1,1,10], vec![2,2,9]], 7);
        assert_eq!(ans, 12);
    }
}