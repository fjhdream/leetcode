pub fn min_stone_sum(piles: Vec<i32>, mut k: i32) -> i32 {
    // 优先队列大根堆
    let mut heap = std::collections::BinaryHeap::new();
    for pile in piles {
        heap.push(pile);
    }
    while k > 0 && heap.len() >= 1 {
        let pile = heap.pop().unwrap();
        // 移除 pile floor(pile / 2)
        heap.push(pile - pile / 2);
        k -= 1;
    }
    let mut sum = 0;
    while let Some(pile) = heap.pop() {
        sum += pile;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let piles = vec![5, 4, 9];
        assert_eq!(min_stone_sum(piles, 2), 12);
    }

    #[test]
    fn it_works2() {
        let piles = vec![4, 3, 6, 7];
        assert_eq!(min_stone_sum(piles, 3), 12);
    }

    #[test]
    fn it_works3() {
        let piles = vec![100];
        assert_eq!(min_stone_sum(piles, 1), 50);
    }
}
