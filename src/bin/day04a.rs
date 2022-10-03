#[derive(Debug)]
pub struct InputError;

fn has_double_digits_at_least_once(number: i32) -> bool {
    let str = number.to_string();
    let mut chars = str.chars();
    match chars.next() {
        None => false,
        Some(first_c) => {
            let mut last_c = first_c;
            for c in chars {
                if c == last_c {
                    return true
                }
                last_c = c;
            }
            false
        }
    }
}

fn digits_form_nondecreasing_sequence(number: i32) -> bool {
    let str = number.to_string();
    let mut chars = str.chars();
    match chars.next() {
        None => false,
        Some(first_c) => {
            let mut last_c = first_c;
            for c in chars {
                if c < last_c {
                    return false
                }
                last_c = c;
            }
            true
        }
    }
}

fn main() {
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line);
    let mut spl = line.trim().split('-');
    let lbound = spl.next().ok_or(InputError)
                           .and_then(|str| str.parse::<i32>().map_err(|_| InputError))
                           .unwrap();
    let ubound = spl.next().ok_or(InputError)
                           .and_then(|str| str.parse::<i32>().map_err(|_| InputError))
                           .unwrap();

    let count = (lbound..ubound).filter(|number| has_double_digits_at_least_once(*number) &&
                                                 digits_form_nondecreasing_sequence(*number))
                                .count();

    println!("{count}");
}
