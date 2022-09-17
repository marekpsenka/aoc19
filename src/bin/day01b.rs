fn calculate_fuel(payload: i64) -> i64 {
    let req = (payload as f64 / 3.0).floor() as i64 - 2;
    match req {
        x if {x <= 0} => 0,
        _ => req + calculate_fuel(req)
    }
}

fn main() {
    let mut total = 0_i64;
    for line in std::io::stdin().lines() {
        let weight = line.unwrap().parse::<i64>().unwrap();
        total += calculate_fuel(weight)
    }

    println!("{total}");
}
