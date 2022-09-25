pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
    let mut x = tx;
    let mut y = ty;
    while x > sx && y > sy && x != y{
        if x > y {
            x %= y;
        } else {
            y %= x;
        }
    }
    if x == sx && y == sy {
        true
    } else if x == sx {
        y > sy && (y - sy) % sx == 0
    } else if y == sy {
        x > sx && (x - sx) % sy == 0
    } else {
        false
    }
}