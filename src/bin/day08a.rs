fn count_occurences(cs: &[char], c: char) -> usize {
    cs.iter().filter(|&other_c| *other_c == c).count()
}

fn main() {
    let width : usize = 25;
    let height : usize = 6;
    let mut line = String::new();
    _ = std::io::stdin().read_line(&mut line);

    let chars = line.trim().chars().collect::<Vec<char>>();

    let min_chunk = chars.chunks(width * height)
                         .min_by_key(|&chunk| count_occurences(chunk, '0'))
                         .unwrap();

    let result = count_occurences(min_chunk, '1')
        * count_occurences(min_chunk, '2');

    println!("{result}");
}