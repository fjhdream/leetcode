use std::vec;

pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    items.sort_by(|item1, item2| item1[0].cmp(&item2[0]));
    (1..items.len()).for_each(|idx| items[idx][1] = items[idx][1].max(items[idx - 1][1]));
    let mut ans = vec![];
    queries.iter().for_each(|query_price| ans.push(query(&items, *query_price)));
    ans
}

fn query(items: &Vec<Vec<i32>>, query: i32) -> i32 {
    let (mut l, mut r) = (0usize, items.len());
    while l < r {
        let mid = l + (r - l) / 2;
        if items[mid][0] > query {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    return if l == 0 { 0 } else { items[l - 1][1] };
}

#[test]
fn test() {
    let items = vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]];
    let queries = vec![1, 2, 3, 4, 5, 6];
    let ans = maximum_beauty(items, queries);
    assert_eq!(ans, vec![2, 4, 5, 5, 6, 6]);
}
