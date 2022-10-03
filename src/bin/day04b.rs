#[derive(Debug)]
pub struct InputError;
fn has_exact_double_digits_at_least_once(number: i32) -> bool {
    let str = number.to_string();
    let mut chars = str.chars().peekable();
    while let Some(this_c) = chars.next() {
        let mut rep = 1;
        while chars.next_if(|c| *c == this_c).is_some()
        {
            rep += 1;
        }
        if rep == 2 {
            return true;
        }
    }

    false
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

    let count = (lbound..ubound).filter(|number| has_exact_double_digits_at_least_once(*number) &&
                                                 digits_form_nondecreasing_sequence(*number))
                                .count();
    println!("{count}");
}
