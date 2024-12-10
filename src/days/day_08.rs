use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    iter,
};

use aoclib_rs::{pairwise_iter, printwriteln};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
}

pub fn run() {
    let write_file = File::create("outputs/08.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/08.txt").unwrap();
    let contents: Vec<&[u8]> = contents.trim().split('\n').map(|s| s.as_bytes()).collect();

    let mut m: HashMap<u8, Vec<Point>> = HashMap::new();
    for (y, row) in contents.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != b'.' {
                m.entry(*cell)
                    .and_modify(|e| e.push(Point::new(x, y)))
                    .or_insert(vec![Point::new(x, y)]);
            }
        }
    }

    part1(&mut writer, &contents, &m);
    part2(&mut writer, &contents, &m);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &[&[u8]], m: &HashMap<u8, Vec<Point>>) {
    let mut antinodes = HashSet::new();
    let mut total = 0;
    for p in pairwise_iter_hm(m) {
        let mut f = |antinode| {
            if is_valid_point(contents, antinode) {
                if !antinodes.contains(&antinode) {
                    total += 1;
                }
                antinodes.insert(antinode);
            }
        };

        f(find_antinode(p.0, p.1));
        f(find_antinode(p.1, p.0));
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[&[u8]], m: &HashMap<u8, Vec<Point>>) {
    let mut antinodes = HashSet::new();
    let mut total = 0;

    let mut f = |p1, p2| {
        let (mut curr, mut next) = (p1, p2);
        loop {
            if is_valid_point(contents, next) {
                if !antinodes.contains(&next) {
                    total += 1;
                }
                antinodes.insert(next);
            } else {
                break;
            }

            (curr, next) = (next, find_antinode(curr, next))
        }
    };

    for p in pairwise_iter_hm(m) {
        f(p.0, p.1);
        f(p.1, p.0);
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

fn find_antinode(point_1: Point, point_2: Point) -> Point {
    let d_x = point_2.x - point_1.x;
    let d_y = point_2.y - point_1.y;

    Point {
        x: point_2.x + d_x,
        y: point_2.y + d_y,
    }
}

fn is_valid_point(contents: &[&[u8]], p: Point) -> bool {
    p.x >= 0 && p.x < contents[0].len() as i32 && p.y >= 0 && p.y < contents.len() as i32
}

fn pairwise_iter_hm(m: &HashMap<u8, Vec<Point>>) -> impl Iterator<Item = (Point, Point)> + use<'_> {
    let mut it: Box<dyn Iterator<Item = (Point, Point)>> = Box::new(iter::empty());
    for v in m.values() {
        it = Box::new(it.chain(pairwise_iter(v)));
    }

    it
}
