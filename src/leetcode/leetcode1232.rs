pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let mut coordinates = coordinates; 
    if coordinates.len() <= 2 {
        return true;
    }
    let delta_x = coordinates[0][0];
    let delta_y = coordinates[0][1];
    let n = coordinates.len();
    for idx in 0..n {
        coordinates[idx][0] -= delta_x;
        coordinates[idx][1] -= delta_y;
    }
    let a = coordinates[1][1];
    let b = -coordinates[1][0];
    for idx in 2..n {
        let x = coordinates[idx][0];
        let y = coordinates[idx][1];
        if a * x + b * y != 0 {
            return false;
        }
    }
    true
}