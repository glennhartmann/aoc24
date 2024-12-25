use std::{
    collections::HashMap,
    fmt,
    io::{BufWriter, Write},
    iter,
};

use aoclib_rs::{
    dijkstra::{Dijkstrable, PqElement},
    dir::Direction,
    prep_io, printwriteln, u8_to_string,
};

#[derive(Copy, Clone)]
struct Point {
    val: u8,
    dist: Option<u32>,
}

impl Point {
    fn new(val: u8) -> Point {
        Point { val, dist: None }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.dist.is_none() {
            write!(f, "({}, -1)", u8_to_string(self.val))
        } else {
            write!(f, "({}, {:02})", u8_to_string(self.val), self.dist.unwrap())
        }
    }
}

struct Points<'a>(&'a mut Vec<Vec<Point>>);

impl Dijkstrable for Points<'_> {
    type Point = (usize, usize);
    type Bounds = (usize, usize);
    type Dist = u32;
    type PQE = PqElement<(usize, usize), u32>;

    fn neighbours(
        p: Self::Point,
        b: Self::Bounds,
    ) -> impl Iterator<Item = (Self::Point, Self::Dist)> {
        iter::zip(Direction::iter_valid_usizes_deltas(p, b), iter::repeat(1))
    }

    fn is_impossible(&self, p: Self::Point) -> bool {
        self.0[p.1][p.0].val == b'#'
    }

    fn dist(&self, p: Self::Point) -> Option<Self::Dist> {
        self.0[p.1][p.0].dist
    }

    fn set_dist(&mut self, p: Self::Point, dist: Option<Self::Dist>) {
        self.0[p.1][p.0].dist = dist;
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 20).unwrap();
    let contents: Vec<&[u8]> = contents.iter().map(|s| s.as_bytes()).collect();

    let mut points: Vec<Vec<Point>> = Vec::with_capacity(contents.len());
    for row in contents {
        let mut points_row = Vec::with_capacity(row.len());
        for cell in row {
            points_row.push(Point::new(*cell));
        }
        points.push(points_row);
    }

    part1(&mut writer, &mut points);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, points: &mut Vec<Vec<Point>>) {
    let end = find_start_end(points, b'E');
    points[end.1][end.0].dist = Some(0);

    let bounds = (points.len(), points[0].len());
    Points(points).dijkstra(end, 0, bounds);

    println!("{:?}", points);

    let mut cheats = HashMap::new();
    let mut over100 = 0;
    for (y, row) in points.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.val == b'#' || cell.dist.is_none() {
                continue;
            }

            for n1 in Direction::iter_valid_usizes_deltas((x, y), (points.len(), row.len())) {
                for n2 in
                    Direction::iter_valid_usizes_deltas((n1.0, n1.1), (points.len(), row.len()))
                {
                    let n2cell = points[n2.1][n2.0];
                    if n2cell.val != b'#' && n2cell.dist.is_some() {
                        let dist = cell.dist.unwrap() as i32 - n2cell.dist.unwrap() as i32 - 2;
                        if dist > 0 {
                            println!("({}, {}) -> ({}, {}) saves {}", x, y, n2.0, n2.1, dist);
                            cheats.entry(dist).and_modify(|i| *i += 1).or_insert(1);

                            if dist >= 100 {
                                over100 += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    for (dist, count) in cheats {
        println!("{} cheats that save {}", count, dist);
    }

    printwriteln!(writer, "part 1: {}", over100).unwrap();
}

fn find_start_end(map: &[Vec<Point>], symbol: u8) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.val == symbol {
                return (x, y);
            }
        }
    }

    panic!("start not found");
}
