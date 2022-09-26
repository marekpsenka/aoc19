use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Motion {
    direction: Direction,
    dist: i32,
}

#[derive(Debug)]
pub struct ParseMotionError;

fn parse_motion(s: &str) -> Result<Motion, ParseMotionError> {
    if s.len() <= 1 {
        Err(ParseMotionError)
    } else {
        match s.as_bytes()[0] as char {
            // TODO: Can code not be duplicated in this match body?
            'U' => s[1..]
                .parse::<i32>()
                .map_err(|_| ParseMotionError {})
                .map(|dist| Motion {
                    direction: Direction::Up,
                    dist,
                }),
            'D' => s[1..]
                .parse::<i32>()
                .map_err(|_| ParseMotionError {})
                .map(|dist| Motion {
                    direction: Direction::Down,
                    dist,
                }),
            'L' => s[1..]
                .parse::<i32>()
                .map_err(|_| ParseMotionError {})
                .map(|dist| Motion {
                    direction: Direction::Left,
                    dist,
                }),
            'R' => s[1..]
                .parse::<i32>()
                .map_err(|_| ParseMotionError {})
                .map(|dist| Motion {
                    direction: Direction::Right,
                    dist,
                }),
            _ => Err(ParseMotionError),
        }
    }
}

fn parse_motions(s: &str) -> Result<Vec<Motion>, ParseMotionError> {
    s.split(',').map(parse_motion).collect()
}

#[derive(Debug)]
pub struct BuildMapError {}

fn build_map(motions: &[Motion]) -> Result<HashMap<Point, i32>, BuildMapError> {
    let mut map = HashMap::<Point, i32>::new();
    let mut p = Point(0, 0);
    let mut steps = 0i32;

    for m in motions {
        match m.direction {
            Direction::Up => {
                for _ in 0..m.dist {
                    _ = map.insert(p, steps);
                    p.1 += 1;
                    steps += 1;
                }
            },
            Direction::Down => {
                for _ in 0..m.dist {
                    _ = map.insert(p, steps);
                    p.1 -= 1;
                    steps += 1;
                }
            },
            Direction::Left => {
                for _ in 0..m.dist {
                    _ = map.insert(p, steps);
                    p.0 -= 1;
                    steps += 1;
                }
            },
            Direction::Right => {
                for _ in 0..m.dist {
                    _ = map.insert(p, steps);
                    p.0 += 1;
                    steps += 1;
                }
            }
        }
    };

    Ok(map)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Point(i32, i32);

fn main() {
    let mut fst_line = String::new();
    _ = std::io::stdin().read_line(&mut fst_line);
    let mut snd_line = String::new();
    _ = std::io::stdin().read_line(&mut snd_line);
    let fst_map = parse_motions(fst_line.trim())
                    .map_err(|_| BuildMapError {})
                    .and_then(|motions| build_map(&motions))
                    .unwrap();
    let snd_map = parse_motions(snd_line.trim())
                    .map_err(|_| BuildMapError {})
                    .and_then(|motions| build_map(&motions))
                    .unwrap();

    let fst_set: HashSet<Point> = fst_map.keys().cloned().collect();
    let snd_set: HashSet<Point> = snd_map.keys().cloned().collect();

    let mut min_steps = i32::MAX;
    for p in fst_set.intersection(&snd_set) {
        let steps = fst_map[p] + snd_map[p];
        if steps > 0 && steps < min_steps {
            min_steps = steps;
        }
    }

    println!("{min_steps}");
}
