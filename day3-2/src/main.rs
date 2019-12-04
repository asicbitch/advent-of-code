use std::io;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(PartialEq)]
enum Orientation {
    Clockwise,
    Counterclockwise,
    Colinear,
}

struct Segment {
    dir: Direction,
    length: u32,
}

#[derive(Clone, PartialEq, Copy)]
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

fn endpoints_on_segments(wire: &Vec<Segment>) -> Vec<(Point, Point)> {
    let mut result: Vec<(Point, Point)> = Vec::new();
    let mut end_point = Point { x: 0, y: 0 };
    for seg in wire {
        let start_point = end_point.clone();
        match seg.dir {
            Direction::Up => end_point.y = end_point.y + seg.length as i32,
            Direction::Down => end_point.y = end_point.y - seg.length as i32,
            Direction::Right => end_point.x = end_point.x + seg.length as i32,
            Direction::Left => end_point.x = end_point.x - seg.length as i32,
        }
        result.push((start_point, end_point));
    }
    result
}

fn get_orientation(p1: Point, p2: Point, p3: Point) -> Orientation {
    let result = (p2.y - p1.y) * (p3.x - p2.x) - (p2.x - p1.x) * (p3.y - p2.y);
    if result > 0 {
        return Orientation::Clockwise;
    } else if result < 0 {
        return Orientation::Counterclockwise;
    } else {
        return Orientation::Colinear;
    }
}

fn find_intersection(wire_a: (Point, Point), wire_b: (Point, Point)) -> Option<Point> {
    let o1 = get_orientation(wire_a.0, wire_a.1, wire_b.0);
    let o2 = get_orientation(wire_a.0, wire_a.1, wire_b.1);
    let o3 = get_orientation(wire_b.0, wire_b.1, wire_a.0);
    let o4 = get_orientation(wire_b.0, wire_b.1, wire_a.1);

    if (o1 != o2) && (o3 != o4) {
        if wire_a.0.x == wire_a.1.x {
            return Some(Point {
                x: wire_a.0.x,
                y: wire_b.0.y,
            });
        } else {
            return Some(Point {
                x: wire_b.0.x,
                y: wire_a.0.y,
            });
        }
    }
    None
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

    // each wire segment in terms of its endpoints
    let first_points = endpoints_on_segments(&first_wire);
    let second_points = endpoints_on_segments(&second_wire);

    // find intersections
    let mut combined_distance = 0;
    let mut temp_distance: i32;
    let mut first_wire_length = 0;
    let mut second_wire_length: i32;
    for n in 0..first_points.len() {
        second_wire_length = 0;
        let points_temp = second_points.clone();
        for m in 0..points_temp.len() {
            match find_intersection(first_points[n], points_temp[m]) {
                Some(val) => {
                    if first_points[n].0.x == first_points[n].1.x {
                        temp_distance = first_wire_length
                            + (first_points[n].0.y - val.y).abs()
                            + second_wire_length
                            + (points_temp[m].0.x - val.x).abs();
                    } else {
                        temp_distance = first_wire_length
                            + (first_points[n].0.x - val.x).abs()
                            + second_wire_length
                            + (points_temp[m].0.y - val.y).abs();
                    }
                    if (temp_distance < combined_distance) || (combined_distance == 0) {
                        combined_distance = temp_distance;
                    }
                }
                None => {}
            }
            second_wire_length = second_wire_length + second_wire[m].length as i32;
        }
        first_wire_length = first_wire_length + first_wire[n].length as i32;
    }

    println!("Smallest combined distance is: {}", combined_distance);
}
