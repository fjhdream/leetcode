pub fn distance_between_bus_stops(distance: Vec<i32>, mut start: i32, mut destination: i32) -> i32 {
    if start > destination {
        let tmp = destination;
        destination = start;
        start = tmp;
    }
    let start = start as usize;
    let destination = destination as usize;
    let sum: i32 = distance.iter().sum();
    let mut dis = 0;
    for i in start..destination {
        dis += distance[i];
    }
    return dis.min(sum - dis);
}
