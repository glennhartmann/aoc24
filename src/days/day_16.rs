use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    io::{BufWriter, Write},
};

use aoclib_rs::{dir::Direction, prep_io, printwriteln};

#[derive(Copy, Clone)]
struct Node {
    val: u8,
    distance: u32,
    visited: bool,
}

impl Node {
    fn new(val: u8) -> Node {
        Node {
            val,
            distance: u32::MAX,
            visited: false,
        }
    }
}

#[derive(Copy, Clone)]
struct PqElement((usize, usize, Direction), u32);

impl Ord for PqElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for PqElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PqElement {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for PqElement {}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 16).unwrap();
    let mut contents: Vec<Vec<Vec<Node>>> = contents
        .iter()
        .map(|line| {
            let by = line.as_bytes();
            by.iter().map(|b| vec![Node::new(*b); 4]).collect()
        })
        .collect();

    part1(&mut writer, &mut contents);
    part2(&mut writer, &mut contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, map: &mut [Vec<Vec<Node>>]) {
    let start = find_start_end(map, b'S');
    let end = find_start_end(map, b'E');
    dijkstra(map, (start.0, start.1, Direction::Right));
    printwriteln!(
        writer,
        "part 1: {}",
        map[end.1][end.0][dir_to_usize(Direction::Right)].distance
    )
    .unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, map: &mut Vec<Vec<Vec<Node>>>) {
    let mut hs: HashSet<(usize, usize)> = HashSet::new();
    let end = find_start_end(map, b'E');
    hs.insert(find_start_end(map, b'S'));
    hs.insert(end);
    compute_cells_on_path_rec(map, (end.0, end.1, Direction::Right), &mut hs);

    printwriteln!(writer, "part 2: {}", hs.len()).unwrap();
}

fn compute_cells_on_path_rec(
    map: &mut Vec<Vec<Vec<Node>>>,
    loc: (usize, usize, Direction),
    hs: &mut HashSet<(usize, usize)>,
) {
    if map[loc.1][loc.0][dir_to_usize(loc.2)].visited {
        return;
    }

    map[loc.1][loc.0][dir_to_usize(loc.2)].visited = true;

    for n in neighbours_reverse(loc) {
        if map[n.1][n.0][dir_to_usize(n.2)].distance
            < map[loc.1][loc.0][dir_to_usize(loc.2)].distance
        {
            hs.insert((n.0, n.1));
            compute_cells_on_path_rec(map, (n.0, n.1, n.2), hs);
        }
    }
}

fn dir_to_usize(dir: Direction) -> usize {
    match dir {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    }
}

fn dijkstra(map: &mut [Vec<Vec<Node>>], start: (usize, usize, Direction)) -> u32 {
    let mut q = BinaryHeap::new();
    q.push(Reverse(PqElement(start, 0)));

    while !q.is_empty() {
        let curr = q.pop().unwrap();

        for n in neighbours(curr.0 .0) {
            let d = if map[n.1][n.0][dir_to_usize(n.2)].val == b'#' {
                u32::MAX
            } else {
                curr.0 .1 + n.3
            };
            if d < map[n.1][n.0][dir_to_usize(n.2)].distance {
                map[n.1][n.0][dir_to_usize(n.2)].distance = d;
                q.push(Reverse(PqElement((n.0, n.1, n.2), d)));
            }
        }
    }

    u32::MAX
}

fn neighbours(p: (usize, usize, Direction)) -> Vec<(usize, usize, Direction, u32)> {
    let straight = p.2.apply_delta_to_usizes((p.0, p.1));

    vec![
        (p.0, p.1, p.2.rotate_right(), 1000),
        (p.0, p.1, p.2.rotate_left(), 1000),
        (straight.0, straight.1, p.2, 1),
    ]
}

fn neighbours_reverse(p: (usize, usize, Direction)) -> Vec<(usize, usize, Direction, u32)> {
    let mut n = neighbours(p);
    (n[2].0, n[2].1) = p.2.opposite().apply_delta_to_usizes((p.0, p.1));

    n
}

fn find_start_end(map: &[Vec<Vec<Node>>], symbol: u8) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell[0].val == symbol {
                return (x, y);
            }
        }
    }

    panic!("start not found");
}
