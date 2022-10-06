pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
    let sum = arr.iter().sum::<i32>();
    if sum % 3 != 0 {
        return vec![-1, -1];
    }
    if sum == 0 {
        return vec![0, 2];
    }
    let partial = sum / 3;
    let (mut first, mut second, mut third, mut cur) = (0, 0, 0, 0);
    (0..arr.len()).for_each(|i| {
        if arr[i] == 1 {
            if cur == 0 {
                first = i;
            } else if cur == partial {
                second = i;
            } else if cur == 2 * partial {
                third = i;
            }
            cur += 1;
        }
    });
    let len = arr.len() - third;
    if first + len <= second && second + len <= third {
        let mut i = 0;
        while third + i < arr.len() {
            if arr[first + i] != arr[second + i] || arr[second + i] != arr[third + i] {
                return vec![-1, -1];
            }
            i += 1;
        }
        return vec![first as i32 + len as i32 - 1, second as i32 + len as i32];
    }
    vec![-1, -1]
}

#[test]
pub fn test() {
    let ans = three_equal_parts(vec![1, 0, 1, 0, 1]);
    assert_eq!(ans, vec![0, 3]);
}
