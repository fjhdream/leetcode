fn main() {
    let mut num = 0;
    for big_meal in 2..=13734i32 {
        if big_meal.count_ones() == 1 {
            num += 1;
            println!("true {}", big_meal);
        } else {
            println!("false {}", big_meal);
        }
    }
    println!("true num is ({})", num);
}
