fn main() {
    let mut total = 0_i64;
    for line in std::io::stdin().lines() {
        let weight = line.unwrap().parse::<i32>().unwrap();
        total += (weight as f64 / 3.0).floor() as i64 - 2
    }

    println!("{total}");
}
