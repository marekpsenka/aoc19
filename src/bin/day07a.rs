use aoc19::intcode;
use itertools::Itertools;

fn main() {
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line);
    let code = intcode::parse_code(&line).unwrap();
    let phases = vec![0, 1, 2, 3, 4];
    let mut max_signal = i32::MIN;
    for phases_p in phases.iter().permutations(phases.len())
    {
        let mut signal = 0;
        for phase in phases_p
        {
            let mut computer = intcode::Computer::new(&code, vec![*phase, signal]);
            computer.run().unwrap();
            signal = computer.output()[0];
        }

        if signal > max_signal
        {
            max_signal = signal;
        }
    }

    println!("{max_signal}")
}