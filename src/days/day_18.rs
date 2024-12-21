use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    io::{BufWriter, Write},
};

use aoclib_rs::{dir::Direction, prep_io, printwriteln, split_and_parse, u8_to_string};

const WIDTH: usize = 71;
const HEIGHT: usize = 71;

#[derive(Copy, Clone)]
struct Node {
    val: u8,
    distance: u32,
}

impl Node {
    fn new() -> Node {
        Node {
            val: b'.',
            distance: u32::MAX,
        }
    }
}

#[derive(Copy, Clone)]
struct PqElement {
    x: usize,
    y: usize,
    val: u32,
}

impl Ord for PqElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for PqElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PqElement {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for PqElement {}

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
    mp[0][0].distance = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", u8_to_string(*map.entry((x, y)).or_insert(b'.')));
        }
        println!();
    }

    let shortest_path = dijkstra(&mut mp);
    printwriteln!(writer, "part 1: {}", shortest_path).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<usize>]) {
    let mut mp = vec![vec![Node::new(); WIDTH]; HEIGHT];
    let mut map = HashMap::new();
    for point in contents[..=1024].iter() {
        map.insert((point[0], point[1]), b'#');
        mp[point[1]][point[0]].val = b'#';
    }
    mp[0][0].distance = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", u8_to_string(*map.entry((x, y)).or_insert(b'.')));
        }
        println!();
    }

    let mut curr = 1024;
    loop {
        let shortest_path = dijkstra(&mut mp.clone());
        if shortest_path == u32::MAX {
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

fn dijkstra(map: &mut [Vec<Node>]) -> u32 {
    let mut q = BinaryHeap::new();
    q.push(Reverse(PqElement { x: 0, y: 0, val: 0 }));

    while !q.is_empty() {
        let curr = q.pop().unwrap();

        if curr.0.x == WIDTH - 1 && curr.0.y == HEIGHT - 1 {
            return curr.0.val;
        }

        for n in Direction::iter_valid_usizes_deltas((curr.0.x, curr.0.y), (WIDTH, HEIGHT)) {
            let d = if map[n.1][n.0].val == b'#' {
                u32::MAX
            } else {
                curr.0.val + 1
            };
            if d < map[n.1][n.0].distance {
                map[n.1][n.0].distance = d;
                q.push(Reverse(PqElement {
                    x: n.0,
                    y: n.1,
                    val: d,
                }));
            }
        }
    }

    u32::MAX
}
