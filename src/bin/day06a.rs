use std::collections::HashMap;
use std::io::stdin;

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
pub struct CountOrbitsError;

fn count_orbits(s: &str, m: &MapData) -> Result<i32, CountOrbitsError> {
    match s {
        "COM" => Ok(0),
        s2 if { m.contains_key(s2) } => {
            count_orbits(&m[s2], m).map(|i| i + 1)
        },
        _ => Err(CountOrbitsError)
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
        .map_err(|_| CountOrbitsError {})
        .and_then(|m| {
            m.keys()
             .map(|k| count_orbits(k, &m))
             .collect::<Result<Vec<i32>, CountOrbitsError>>()
        })
        .map(|v| v.into_iter().sum::<i32>())
        .unwrap();

    println!("{}", result)
}
