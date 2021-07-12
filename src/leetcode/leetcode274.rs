pub fn h_index(mut citations: Vec<i32>) -> i32 {
    let (mut h, n, mut i) =  (0, citations.len(), 0);
    citations.sort_by(|a, b| a.cmp(b).reverse());
    while i < n && citations[i] > h {
        i += 1;
        h += 1;
    }
    h
}

#[test]
fn test_example() {
    let citations = vec![3,0,6,1,5];
    let ans = h_index(citations);
    assert_eq!(ans, 3);
}