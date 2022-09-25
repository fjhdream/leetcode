use rand::Rng;

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        return Solution {
            radius,
            x_center,
            y_center,
        };
    }

    fn rand_point(&self) -> Vec<f64> {
        loop {
            let x = rand::thread_rng().gen_range(-self.radius, self.radius);
            let y = rand::thread_rng().gen_range(-self.radius, self.radius);
            if x * x + y * y <= self.radius * self.radius {
                return vec![x + self.x_center, y + self.y_center];
            }
        }
    }
}
