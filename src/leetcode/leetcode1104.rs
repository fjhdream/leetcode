pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
    let mut row = 1;
    while 2i32.pow(row as u32) <= label {
        row = row + 1;
    }

    let mut label = label;
    if row % 2 == 0 {
        label = get_reverse(label, row);
    }

    let mut ans = vec![];
    while row > 0 {
        if row % 2 == 0 {
            ans.push(get_reverse(label, row));
        } else {
            ans.push(label);
        }
        label = label / 2;
        row = row - 1;
    }

    ans.reverse();
    ans
}

fn get_reverse(lable: i32, row: i32) -> i32 {
    2i32.pow(row as u32) - 1 - lable + 2i32.pow(row as u32 - 1)
}

#[test]
fn test_example() {
    let label = 1;
    let ans = path_in_zig_zag_tree(label);
    println!("{:?}", ans);
}