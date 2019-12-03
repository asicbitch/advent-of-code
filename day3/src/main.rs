use std::io;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Segment {
    dir: Direction,
    length: u32,
}

#[derive(Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn segments_from_line(line: String) -> Vec<Segment> {
    let mut result: Vec<Segment> = Vec::new();
    for val in line.split(',') {
        let dir: Direction = match val.chars().next() {
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some('R') => Direction::Right,
            Some('L') => Direction::Left,
            _ => {
                println!("Invalid direction!");
                break;
            }
        };
        let length: u32 = match val[1..].trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid length!");
                break;
            }
        };
        result.push(Segment { dir, length });
    }
    result
}

fn points_on_wire(wire: Vec<Segment>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let mut current_point = Point { x: 0, y: 0 };
    for seg in wire {
        match seg.dir {
            Direction::Up => {
                for _n in 1..seg.length + 1 {
                    current_point.y = current_point.y + 1;
                    result.push(current_point.clone());
                }
            }
            Direction::Down => {
                for _n in 1..seg.length + 1 {
                    current_point.y = current_point.y - 1;
                    result.push(current_point.clone());
                }
            }
            Direction::Right => {
                for _n in 1..seg.length + 1 {
                    current_point.x = current_point.x + 1;
                    result.push(current_point.clone());
                }
            }
            Direction::Left => {
                for _n in 1..seg.length + 1 {
                    current_point.x = current_point.x - 1;
                    result.push(current_point.clone());
                }
            }
        }
    }
    result
}

fn main() {
    // first parse the input lines into vectors representing the wires
    let mut first_string = String::new();
    let mut second_string = String::new();
    io::stdin()
        .read_line(&mut first_string)
        .expect("Failed to read line.");
    io::stdin()
        .read_line(&mut second_string)
        .expect("Failed to read line.");
    let first_wire = segments_from_line(first_string);
    let second_wire = segments_from_line(second_string);

    // get all points on each wire
    let first_points = points_on_wire(first_wire);
    let second_points = points_on_wire(second_wire);

    // find matching points
    let mut common_points: Vec<Point> = Vec::new();
    for pt in first_points {
        if second_points.contains(&pt) {
            common_points.push(pt);
        }
    }

    // find common point with smallest Manhattan distance from origin
    let mut smallest_distance = std::i32::MAX;
    for pt in common_points {
        let distance = pt.x.abs() + pt.y.abs();
        if distance < smallest_distance {
            smallest_distance = distance;
        }
    }

    println!("The smallest distance is: {}", smallest_distance);
}
