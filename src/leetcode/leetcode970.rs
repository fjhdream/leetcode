use std::collections::HashSet;

pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut v1 = 1;
    for i in 0..21 {
        let mut v2 = 1;
        for j in 0..21 {
            let v = v1 + v2;
            if v <= bound {
                set.insert(v);
            } else {
                break;
            }
            v2 *= y;
        }
        if v1 > bound {
            break;
        }
        v1 *= x;
    }
    set.into_iter().collect()
}
