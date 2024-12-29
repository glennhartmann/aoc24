use std::{
    collections::HashMap,
    fmt,
    io::{BufWriter, Write},
    iter,
};

use aoclib_rs::{
    dijkstra::{Dijkstrable, PqElement},
    dir::{Dir4, Direction},
    position_2d, prep_io, printwriteln, u8_to_string,
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
        iter::zip(Dir4::iter_valid_usizes_deltas(p, b), iter::repeat(1))
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

struct PointsPart2<'a>(&'a mut Vec<Vec<Point>>);

impl Dijkstrable for PointsPart2<'_> {
    type Point = (usize, usize);
    type Bounds = (usize, usize);
    type Dist = u32;
    type PQE = PqElement<(usize, usize), u32>;

    fn neighbours(
        p: Self::Point,
        b: Self::Bounds,
    ) -> impl Iterator<Item = (Self::Point, Self::Dist)> {
        iter::zip(Dir4::iter_valid_usizes_deltas(p, b), iter::repeat(1))
    }

    fn is_impossible(&self, _: Self::Point) -> bool {
        false
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
    let points_copy = points.clone();

    let end = position_2d(&points, |cell: &Point| cell.val == b'E').unwrap();
    points[end.1][end.0].dist = Some(0);

    let bounds = (points.len(), points[0].len());
    Points(&mut points).dijkstra(end, 0, bounds);

    println!("{:?}", points);

    part1(&mut writer, &points);
    part2(&mut writer, &points, &points_copy);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, points: &[Vec<Point>]) {
    let mut cheats = HashMap::new();
    let mut over100 = 0;
    for (y, row) in points.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.val == b'#' || cell.dist.is_none() {
                continue;
            }

            for n1 in Dir4::iter_valid_usizes_deltas((x, y), (points.len(), row.len())) {
                for n2 in Dir4::iter_valid_usizes_deltas((n1.0, n1.1), (points.len(), row.len())) {
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

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    points: &[Vec<Point>],
    original_points: &[Vec<Point>],
) {
    let mut cheats = HashMap::new();
    for (y, row) in points.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.val == b'#' || cell.dist.is_none() {
                continue;
            }

            println!("{} {}", x, y);
            let inner_cheats = compute_cheats_20(points, original_points.to_owned(), x, y);
            merge_hash_maps(&mut cheats, &inner_cheats);
        }
    }

    let mut over100 = 0;
    for (dist, count) in cheats {
        if dist < 100 {
            continue;
        }

        over100 += count;
        println!("{} cheats that save {}", count, dist);
    }

    printwriteln!(writer, "part 2: {}", over100).unwrap();
}

fn compute_cheats_20(
    points: &[Vec<Point>],
    mut points_part_2: Vec<Vec<Point>>,
    start_x: usize,
    start_y: usize,
) -> HashMap<i32, u32> {
    let bounds = (points_part_2.len(), points_part_2[0].len());
    PointsPart2(&mut points_part_2).dijkstra((start_x, start_y), 0, bounds);

    let mut cheats = HashMap::new();
    for (y, row) in points_part_2.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if points[y][x].val == b'#' || points[y][x].dist.is_none() {
                continue;
            }

            if cell.dist.is_none() || cell.dist.unwrap() > 20 {
                continue;
            }

            let dist_saved = points[start_y][start_x].dist.unwrap() as i32
                - points[y][x].dist.unwrap() as i32
                - cell.dist.unwrap() as i32;
            if dist_saved <= 0 {
                continue;
            }

            cheats
                .entry(dist_saved)
                .and_modify(|i| *i += 1)
                .or_insert(1);
        }
    }

    cheats
}

fn merge_hash_maps(cheats: &mut HashMap<i32, u32>, other: &HashMap<i32, u32>) {
    for (k, v) in other {
        cheats.entry(*k).and_modify(|i| *i += *v).or_insert(*v);
    }
}
