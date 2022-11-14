use aoc19::intcode::{self, HaltReason};
use itertools::Itertools;

fn main() {
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line);
    let code = intcode::parse_code(&line).unwrap();
    let phases = vec![5, 6, 7, 8, 9];
    let mut max_signal = i32::MIN;

    for phases_p in phases.iter().permutations(phases.len())
    {
        let mut computers = phases_p.into_iter()
                                .map(|ph| intcode::Computer::new(&code, vec![*ph]))
                                .collect::<Vec<intcode::Computer>>();
        let mut signal = 0;
        let mut i = 0;
        loop
        {
            computers[i].push_input(signal);
            let halt_reason = computers[i].run_adv().unwrap();
            if let Some(s) = computers[i].pop_output()
            {
                signal = s;
            }
            else
            {
                panic!("There is no output")
            }
            i += 1;
            if i == phases.len()
            {
                i = 0;
                if halt_reason == HaltReason::Terminate
                {
                    break;
                }
            }
        }

        if signal > max_signal
        {
            max_signal = signal;
        }
    }

    println!("{max_signal}")
}