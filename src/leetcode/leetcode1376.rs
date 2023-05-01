use std::collections::HashMap;

pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut manager_to_employee_map = HashMap::new();
    let mut root = -1;
    for (i, &employee) in manager.iter().enumerate() {
        if employee == -1 {
            root = i as i32;
        } else {
            manager_to_employee_map
                .entry(employee)
                .or_insert(Vec::new())
                .push(i as i32);
        }
    }
    return dfs(root, &manager_to_employee_map, &inform_time);
}

fn dfs(root: i32, map: &HashMap<i32, Vec<i32>>, time: &Vec<i32>) -> i32 {
    let cur_notify = *time.get(root as usize).unwrap();
    let employee_list_option = map.get(&root);
    if employee_list_option.is_none() {
        return cur_notify;
    }
    let employee_list = employee_list_option.unwrap();
    let mut max = 0;
    for employee in employee_list {
        let employee_notify = dfs(*employee, map, time);
        max = max.max(cur_notify + employee_notify);
    }
    return max;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let ans = num_of_minutes(
            15,
            0,
            vec![-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6],
            vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        assert_eq!(ans, 3);
    }

    fn test2() {
        let ans = num_of_minutes(
            11,
            4,
            vec![5, 9, 6, 10, -1, 8, 9, 1, 9, 3, 4],
            vec![0, 213, 0, 253, 686, 170, 975, 0, 261, 309, 337],
        );
        assert_eq!(ans, 2560);
    }
}
