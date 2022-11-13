use std::collections::HashMap;
use std::io::stdin;
use std::iter;

#[derive(Debug)]
pub struct ParseInputError;

type MapData = HashMap<String, String>;

fn read_orbit(m: &mut MapData, line: &str) -> Result<(), ParseInputError> {
    let mut it = line.trim().split(')');
    it.next()
        .ok_or(ParseInputError)
        .and_then(|fst| it.next().ok_or(ParseInputError).map(|snd| (fst, snd)))
        .and_then(|(fst, snd)| {
            if m.insert(String::from(snd), String::from(fst)).is_some() {
                Err(ParseInputError)
            } else {
                Ok(())
            }
        })
}

#[derive(Debug)]
pub struct AssemblePathError;
fn assemble_path<'a>(s: &'a str, m: &'a MapData) -> Result<Vec<&'a str>, AssemblePathError> {
    match s {
        "COM" => Ok(vec![]),
        s2 if { m.contains_key(s2) } => {
            assemble_path(&m[s2], m).map(|mut v2| { v2.push(s); v2 })
        },
        _ => Err(AssemblePathError)
    }

}

fn main() {
    let result = stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| ParseInputError {})
        .map(|lines| {
            let mut m = MapData::new();
            lines
                .into_iter()
                .try_for_each(|line| read_orbit(&mut m, &line))
                .unwrap();
            m
        })
        .map(|m| {
            let v1 = assemble_path("YOU", &m).unwrap();
            let v2 = assemble_path("SAN", &m).unwrap();
            let same_count = iter::zip(v1.iter(), v2.iter()).take_while(|(a, b)| a == b).count();
            v1.len() + v2.len() - 2 * same_count - 2
        })
        .unwrap();

    println!("{}", result)
}
