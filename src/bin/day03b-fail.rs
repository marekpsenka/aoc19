#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

#[derive(Copy, Clone)]
struct Point(i32, i32);

#[derive(Copy, Clone)]
pub struct Segment {
    origin: Point,
    length: i32,
    length_up_to: i32,
}

fn motion_to_segments(
    origin: Point,
    motions: &[Motion],
) -> (Vec<Segment>, Vec<Segment>, Vec<Segment>, Vec<Segment>) {
    let mut upward_segments = Vec::<Segment>::new();
    let mut leftward_segments = Vec::<Segment>::new();
    let mut downward_segments = Vec::<Segment>::new();
    let mut rightward_segments = Vec::<Segment>::new();
    let mut p = origin;
    let mut length_up_to = 0i32;
    for Motion { direction, dist } in motions {
        match direction {
            Direction::Up => {
                upward_segments.push(Segment {
                    origin: p,
                    length: *dist,
                    length_up_to,
                });
                p.1 += dist;
                length_up_to += dist;
            }
            Direction::Down => {
                downward_segments.push(Segment {
                    origin: p,
                    length: *dist,
                    length_up_to,
                });
                p.1 -= dist;
                length_up_to += dist;
            }
            Direction::Right => {
                rightward_segments.push(Segment {
                    origin: p,
                    length: *dist,
                    length_up_to,
                });
                p.0 += dist;
                length_up_to += dist;
            }
            Direction::Left => {
                leftward_segments.push(Segment {
                    origin: p,
                    length: *dist,
                    length_up_to,
                });
                p.0 -= dist;
                length_up_to += dist;
            }
        }
    }

    (
        upward_segments,
        downward_segments,
        leftward_segments,
        rightward_segments,
    )
}

fn left_up_intersection(horizontal: Segment, vertical: Segment) -> Option<i32> {
    if horizontal.origin.1 > vertical.origin.1 + vertical.length {
        return None;
    }
    if horizontal.origin.1 < vertical.origin.1 {
        return None;
    }
    if vertical.origin.0 > horizontal.origin.0 {
        return None;
    }
    if vertical.origin.0 < horizontal.origin.0 - horizontal.length {
        return None;
    }

    Some(
        horizontal.length_up_to + horizontal.origin.0 - vertical.origin.0
            + vertical.length_up_to
            + horizontal.origin.1
            - vertical.origin.1,
    )
}

fn left_down_intersection(horizontal: Segment, vertical: Segment) -> Option<i32> {
    if horizontal.origin.1 > vertical.origin.1 {
        return None;
    }
    if horizontal.origin.1 < vertical.origin.1 - vertical.length {
        return None;
    }
    if vertical.origin.0 > horizontal.origin.0 {
        return None;
    }
    if vertical.origin.0 < horizontal.origin.0 - horizontal.length {
        return None;
    }

    Some(
        horizontal.length_up_to + horizontal.origin.0 - vertical.origin.0
            + vertical.length_up_to
            + vertical.origin.1
            - horizontal.origin.1,
    )
}

fn right_down_intersection(horizontal: Segment, vertical: Segment) -> Option<i32> {
    if horizontal.origin.1 > vertical.origin.1 {
        return None;
    }
    if horizontal.origin.1 < vertical.origin.1 - vertical.length {
        return None;
    }
    if vertical.origin.0 < horizontal.origin.0 {
        return None;
    }
    if vertical.origin.0 > horizontal.origin.0 + horizontal.length {
        return None;
    }

    Some(
        horizontal.length_up_to + vertical.origin.0 - horizontal.origin.0
            + vertical.length_up_to
            + vertical.origin.1
            - horizontal.origin.1,
    )
}

fn right_up_intersection(horizontal: Segment, vertical: Segment) -> Option<i32> {
    if horizontal.origin.1 > vertical.origin.1 + vertical.length {
        return None;
    }
    if horizontal.origin.1 < vertical.origin.1 {
        return None;
    }
    if vertical.origin.0 < horizontal.origin.0 - horizontal.length {
        return None;
    }
    if vertical.origin.0 > horizontal.origin.0 {
        return None;
    }

    Some(
        horizontal.length_up_to + vertical.origin.0 - horizontal.origin.0
            + vertical.length_up_to
            + horizontal.origin.1
            - vertical.origin.1,
    )
}

fn main() {
    let mut fst_line = String::new();
    _ = std::io::stdin().read_line(&mut fst_line);
    let mut snd_line = String::new();
    _ = std::io::stdin().read_line(&mut snd_line);
    let fst_motions = parse_motions(fst_line.trim()).unwrap();
    let snd_motions = parse_motions(snd_line.trim()).unwrap();
    let (fst_up, fst_down, fst_left, fst_right) = motion_to_segments(Point(0, 0), &fst_motions);
    let (snd_up, snd_down, snd_left, snd_right) = motion_to_segments(Point(0, 0), &snd_motions);
    let mut min_length_up_to = i32::MAX;
    for &horizontal in fst_left.as_slice() {
        for &vertical in snd_up.as_slice() {
            match left_up_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in fst_left.as_slice() {
        for &vertical in snd_down.as_slice() {
            match left_down_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in fst_right.as_slice() {
        for &vertical in snd_up.as_slice() {
            match right_up_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in fst_right.as_slice() {
        for &vertical in snd_down.as_slice() {
            match right_down_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in snd_left.as_slice() {
        for &vertical in fst_up.as_slice() {
            match left_up_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in snd_left.as_slice() {
        for &vertical in fst_down.as_slice() {
            match left_down_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in snd_right.as_slice() {
        for &vertical in fst_up.as_slice() {
            match right_up_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }
    for &horizontal in snd_right.as_slice() {
        for &vertical in fst_down.as_slice() {
            match right_down_intersection(horizontal, vertical) {
                Some(length_up_to) if { length_up_to > 0 } => {
                    if length_up_to < min_length_up_to {
                        min_length_up_to = length_up_to;
                    }
                }
                _ => {}
            }
        }
    }

    println!("{min_length_up_to}");
}
