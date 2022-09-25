pub fn next_greater_element(mut n: i32) -> i32 {
    let mut arr = vec![];
    while n > 0 {
        arr.push(n % 10);
        n /= 10;
    }
    arr.reverse();
    
    let n = arr.len();
    if n == 1 {
        return -1;
    }

    let (idx, pre) = find_min_pair(&arr);
    if idx == -1 && pre == -1 {
        return -1; 
    }
    arr.swap(idx as usize, pre as usize);
    let mut start = (idx + 1) as usize;
    let mut end = n - 1;
    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
    }
    let mut ans = 0i64;
    for val in arr {
        ans = ans * 10 + val as i64;
    }
    return if ans <= i32::MAX as i64 {ans as i32} else {-1}
}

fn find_min_pair(arr: &Vec<i32>) -> (i32, i32) {
    let n = arr.len();
    let mut i = n as i32 - 2;
    while i >= 0 && arr[i as usize] >= arr[i as usize + 1] {
        i -= 1;
    }

    if i >= 0 {
        let mut j = n as i32 - 1;
        while j >= i && arr[j as usize] <= arr[i as usize] {
            j -= 1;
        }
        return (i, j);
    }
    return (i, -1);
}

#[test]
fn test() {
    let ans = next_greater_element(12);
    assert_eq!(ans, 21);
}

#[test]
fn test2() {
    let ans = next_greater_element(21);
    assert_eq!(ans, -1);
}

#[test]
fn test3() {
    let ans = next_greater_element(54341132);
    assert_eq!(ans, 54341231);
}

#[test]
fn test4() {
    let ans = next_greater_element(230241);
    assert_eq!(ans, 230412);
}

#[test]
fn test5() {
    let ans = next_greater_element(230287641);
    assert_eq!(ans, 230412678);
}

#[test]
fn test6() {
    let ans = next_greater_element(2147483476);
    assert_eq!(ans, 2147483647);
}