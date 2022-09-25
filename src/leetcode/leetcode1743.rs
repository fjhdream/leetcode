use std::collections::HashMap;

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let n = adjacent_pairs.len() + 1;
    for pair in adjacent_pairs {
        let key_1 = pair[0];
        let key_2 = pair[1];
        let entry_1 = map.entry(key_1).or_insert(Vec::<i32>::new());
        entry_1.push(key_2);
        let entry_2 = map.entry(key_2).or_insert(Vec::<i32>::new());
        entry_2.push(key_1);
    }
    let mut ans:Vec<i32> = vec![0; n];
    for (key, value) in map.iter() {
        if value.len() == 1{
            ans[0] = *key;
            break;
        }
    }

    ans[1] = *(*map.get(&ans[0]).unwrap()).get(0).unwrap();
    for i in 2..n {
        let adj_vec = map.get(&ans[i-1]).unwrap();
        ans[i] = if ans[i-2] == *adj_vec.get(0).unwrap() {
            *adj_vec.get(1).unwrap()
        } else {
            *adj_vec.get(0).unwrap()
        }
    }
    ans
}

#[test]
fn test_example() {
    let adjacent_pairs = vec![vec![2,1], vec![3,4], vec![3,2]];
    let ans = restore_array(adjacent_pairs);
    println!("{:?}", ans);
}