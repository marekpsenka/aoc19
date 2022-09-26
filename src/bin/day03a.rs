enum Motion {
    Up { dist: i32 },
    Down { dist: i32 },
    Left { dist: i32 },
    Right { dist: i32 }
}

#[derive(Debug)]
pub struct ParseMotionError;

fn parse_motion(s: &str) -> Result<Motion, ParseMotionError> {
    if s.len() <= 1 {
        Err(ParseMotionError)
    }
    else {
        match s.as_bytes()[0] as char {
            // TODO: Can code not be duplicated in this match body?
            'U' => s[1..].parse::<i32>()
                       .map_err(|_| ParseMotionError { })
                       .map(|dist| Motion::Up { dist }),
            'D' => s[1..].parse::<i32>()
                       .map_err(|_| ParseMotionError { })
                       .map(|dist| Motion::Down { dist }),
            'L' => s[1..].parse::<i32>()
                       .map_err(|_| ParseMotionError { })
                       .map(|dist| Motion::Left { dist }),
            'R' => s[1..].parse::<i32>()
                       .map_err(|_| ParseMotionError { })
                       .map(|dist| Motion::Right { dist }),
            _ => Err(ParseMotionError)
        }
    }
}

fn parse_motions(s: &str) -> Result<Vec<Motion>, ParseMotionError> {
    s.split(',').map(parse_motion).collect()
}

#[derive(Copy, Clone)]
struct Point(i32, i32);

impl Point {
    fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}


#[derive(Copy, Clone)]
pub struct Segment {
    origin: Point,
    length: i32
}

fn motion_to_segments(origin: Point, motions: &[Motion]) -> (Vec<Segment>, Vec<Segment>) {
    let mut vertical_segments = Vec::<Segment>::new();
    let mut horizontal_segments = Vec::<Segment>::new();
    let mut p = origin;
    for motion in motions {
        match *motion {
            Motion::Up {dist} => {
                vertical_segments.push(Segment { origin: p, length: dist });
                p.1 += dist;
            },
            Motion::Down {dist} => {
                p.1 -= dist;
                vertical_segments.push(Segment { origin: p, length: dist });
            },
            Motion::Right {dist} => {
                horizontal_segments.push(Segment { origin: p, length: dist });
                p.0 += dist;
            },
            Motion::Left {dist} => {
                p.0 -= dist;
                horizontal_segments.push(Segment { origin: p, length: dist });
            }
        }
    }

    (vertical_segments, horizontal_segments)
}

fn intersection(horizontal: Segment, vertical: Segment) -> Option<Point>
{
    if horizontal.origin.1 > vertical.origin.1 + vertical.length {
        return None;
    }
    if horizontal.origin.1 < vertical.origin.1  {
        return None;
    }
    if vertical.origin.0 < horizontal.origin.0  {
        return None;
    }
    if vertical.origin.0 > horizontal.origin.0 + horizontal.length {
        return None;
    }

    Some(Point(vertical.origin.0, horizontal.origin.1))
}

fn main() {
    let mut fst_line = String::new();
    _ = std::io::stdin().read_line(&mut fst_line);
    let mut snd_line = String::new();
    _ = std::io::stdin().read_line(&mut snd_line);
    let fst_motions = parse_motions(fst_line.trim()).unwrap();
    let snd_motions = parse_motions(snd_line.trim()).unwrap();
    let (fst_vertical, fst_horizontal) = motion_to_segments(Point(0, 0), &fst_motions);
    let (snd_vertical, snd_horizontal) = motion_to_segments(Point(0, 0), &snd_motions);
    let mut smallest_dist = i32::MAX;
    for &horizontal in fst_horizontal.as_slice() {
        for &vertical in snd_vertical.as_slice() {
            match intersection(horizontal, vertical) {
                None => continue,
                Some(p) => {
                    match p.manhattan() {
                        0 => {},
                        dist if { dist < smallest_dist } => {
                            smallest_dist = p.manhattan();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    for &horizontal in snd_horizontal.as_slice() {
        for &vertical in fst_vertical.as_slice() {
            match intersection(horizontal, vertical) {
                None => continue,
                Some(p) => {
                    match p.manhattan() {
                        0 => {},
                        dist if { dist < smallest_dist } => {
                            smallest_dist = p.manhattan();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    println!("{smallest_dist}")
}
