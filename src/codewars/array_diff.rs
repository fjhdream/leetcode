fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    if b.is_empty() || a.is_empty() {
        return a;
    }
    let mut ans: Vec<T> = Vec::new();
    for item_a in a {
        let mut needed = false;
        for (idx, item_b) in b.iter().enumerate() {
            if item_a.eq(item_b) {
                break;
            }
            if idx == b.len() - 1 && !item_a.eq(item_b) {
                needed = true;
                break;
            }
        }
        if needed {
            ans.push(item_a);
        }  
    }
    ans
}


#[test]
fn returns_expected() {
    assert_eq!(array_diff(vec![1,2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1,2,2], vec![1]), vec![2,2]);
    assert_eq!(array_diff(vec![1,2,2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1,2,2], vec![]), vec![1,2,2]);
    assert_eq!(array_diff(vec![], vec![1,2]), vec![]);
    assert_eq!(array_diff(vec![1,2,3], vec![1,2]), vec![3]);
}
