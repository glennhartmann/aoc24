use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    iter,
};

use aoclib_rs::{
    dijkstra::{Dijkstrable, PqElement},
    dir::Direction,
    prep_io, printwriteln, split_and_parse, u8_to_string,
};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

#[derive(Copy, Clone)]
struct Node {
    val: u8,
    distance: Option<u32>,
}

impl Node {
    fn new() -> Node {
        Node {
            val: b'.',
            distance: None,
        }
    }
}

struct Map(Vec<Vec<Node>>);

impl Dijkstrable for Map {
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
        self.0[p.1][p.0].distance
    }

    fn set_dist(&mut self, p: Self::Point, dist: Option<Self::Dist>) {
        self.0[p.1][p.0].distance = dist;
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 18).unwrap();
    let contents: Vec<Vec<usize>> = contents
        .iter()
        .map(|line| split_and_parse(line, ",").unwrap())
        .collect();

    part1(&mut writer, &contents[..1024]);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<usize>]) {
    let mut mp = vec![vec![Node::new(); WIDTH]; HEIGHT];
    let mut map = HashMap::new();
    for point in contents {
        map.insert((point[0], point[1]), b'#');
        mp[point[1]][point[0]].val = b'#';
    }
    mp[0][0].distance = Some(0);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", u8_to_string(*map.entry((x, y)).or_insert(b'.')));
        }
        println!();
    }

    let mut mp = Map(mp);
    mp.dijkstra((0, 0), 0, (WIDTH, HEIGHT));
    printwriteln!(
        writer,
        "part 1: {}",
        mp.0[HEIGHT - 1][WIDTH - 1].distance.unwrap()
    )
    .unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<usize>]) {
    let mut mp = vec![vec![Node::new(); WIDTH]; HEIGHT];
    let mut map = HashMap::new();
    for point in contents[..=1024].iter() {
        map.insert((point[0], point[1]), b'#');
        mp[point[1]][point[0]].val = b'#';
    }
    mp[0][0].distance = Some(0);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", u8_to_string(*map.entry((x, y)).or_insert(b'.')));
        }
        println!();
    }

    let mut curr = 1024;
    loop {
        let mut mp2 = Map(mp.clone());
        mp2.dijkstra((0, 0), 0, (WIDTH, HEIGHT));
        if mp2.0[HEIGHT - 1][WIDTH - 1].distance.is_none() {
            break;
        }
        curr += 1;
        let point = &contents[curr];
        mp[point[1]][point[0]].val = b'#';
    }

    for row in mp {
        for cell in row {
            print!("{}", u8_to_string(cell.val));
        }
        println!();
    }

    let point = &contents[curr];
    printwriteln!(writer, "part 2: {} ({},{})", curr, point[0], point[1]).unwrap();
}
