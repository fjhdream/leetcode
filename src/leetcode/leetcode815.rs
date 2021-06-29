use std::collections::{HashMap, VecDeque, HashSet};
use std::option::Option::Some;

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    if source == target { return 0; }

    let mut site_bus_map: HashMap<i32, HashSet<usize>> = HashMap::new();
    for (bus, sites) in routes.iter().enumerate() {
        for site in sites.iter() {
            if site_bus_map.contains_key(site) {
                let buses = site_bus_map.get_mut(site).unwrap();
                buses.insert(bus);
            } else {
                let mut buses = HashSet::new();
                buses.insert(bus);
                site_bus_map.insert(*site, buses);
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(source);
    let mut visited = HashSet::new();
    let mut ans = 0;
    while !queue.is_empty() {
        ans += 1;
        let size = queue.len();
        for _ in 0..size {
            let site = queue.pop_front().unwrap();
            if let Some(buses) = site_bus_map.get(&site){
                for bus in buses  {
                    if visited.contains(bus) {
                        continue;
                    }
                    visited.insert(bus);
                    for target_site in routes[*bus].iter() {
                        if *target_site == target {
                            return ans;
                        }
                        queue.push_back(*target_site);
                    }
                }
            }
        }
    }

    -1
}

#[test]
fn test_example() {
    let routes = vec![vec![1,2,7], vec![3,6,7]];
    let source = 1;
    let target = 6;
    let ans = num_buses_to_destination(routes, source, target);
    assert_eq!(ans, 2);
}
