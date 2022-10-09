use aoc19::intcode;

fn main() {
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line);
    let code = intcode::parse_code(&line).unwrap();
    let mut computer = intcode::Computer::new(&code, vec![1]);
    computer.run().unwrap();
    for i in computer.output() {
        println!("{i}");
    }
}
