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

fn endpoints_on_segments(wire: Vec<Segment>) -> Vec<(Point, Point)> {
    let mut result: Vec<(Point, Point)> = Vec::new();
    let mut end_point = Point { x: 0, y: 0 };
    for seg in wire {
        let start_point = end_point.clone();
        match seg.dir {
            Direction::Up    => end_point.y = end_point.y + seg.length as i32,
            Direction::Down  => end_point.y = end_point.y - seg.length as i32,
            Direction::Right => end_point.x = end_point.x + seg.length as i32,
            Direction::Left  => end_point.x = end_point.x - seg.length as i32,
        }
        println!("Current point: {} {}", end_point.x, end_point.y);
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
            return Some(Point { x: wire_a.0.x, y: wire_b.0.y} );
        } else {
            return Some(Point { x: wire_b.0.x, y: wire_a.0.y} );
        }
    }
    None
    /*if wire_a.0.x == wire_a.1.x {
        if ((wire_b.0.x < wire_a.0.x) && (wire_b.1.x > wire_a.0.x)) ||
            ((wire_b.0.x > wire_a.0.x) && (wire_b.1.x < wire_a.0.x)) &&
            ((wire_b.0.y > wire_a.0.y) && (wire_b.1.y < wire_a.1.y) ||
             (wire_b.0.y < wire_a.0.y) && (wire_b.1.y > wire_a.1.y)) {
                println!("Points A: {} {}, {} {}", wire_a.0.x, wire_a.0.y, wire_a.1.x, wire_a.1.y);
                println!("Points B: {} {}, {} {}", wire_b.0.x, wire_b.0.y, wire_b.1.x, wire_b.1.y);
                return Some(Point { x: wire_a.0.x, y: wire_b.0.y });
        }
    } else {
        if ((wire_b.0.y < wire_a.0.y) && (wire_b.1.y > wire_b.0.y)) ||
            ((wire_b.0.y > wire_a.0.y) && (wire_b.1.y < wire_b.0.y)) &&
            ((wire_b.0.x > wire_a.0.x) && (wire_b.1.x < wire_a.1.x) ||
             (wire_b.0.x < wire_a.0.x) && (wire_b.1.x > wire_a.1.x)) {
                return Some(Point { x: wire_b.0.x, y: wire_a.0.y });
        }
}*/ 
   // return None;
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

    // each wire in terms of it's endpoints 
    let first_points = endpoints_on_segments(first_wire);
    let second_points = endpoints_on_segments(second_wire);

    // find intersections
    let mut intersections: Vec<Point> = Vec::new();
    for first_seg in first_points {
        let points_temp = second_points.clone();
        for second_seg in points_temp {
            match find_intersection(first_seg, second_seg) {
                Some(val) => intersections.push(val),
                None        => continue,
            }
        }
    }

    // find common point with smallest Manhattan distance from origin
    let mut smallest_distance = std::i32::MAX;
    for pt in intersections {
        let distance = pt.x.abs() + pt.y.abs();
        if distance < smallest_distance {
            smallest_distance = distance;
        }
    }

    println!("The smallest distance is: {}", smallest_distance);
}
