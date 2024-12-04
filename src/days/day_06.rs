use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use crate::common::printwriteln;

struct Position {
    c: u8,
    visited: bool,
}

impl Position {
    fn new(c: u8) -> Position {
        Position { c, visited: false }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn rotate_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub fn run() {
    let write_file = File::create("outputs/06.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/06.txt").unwrap();
    let mut contents: Vec<Vec<Position>> = contents
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().iter().map(|b| Position::new(*b)).collect())
        .collect();

    part1(&mut writer, &mut contents);
    //part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &mut Vec<Vec<Position>>) {
    let (mut x, mut y) = find_start(contents);
    contents[y as usize][x as usize].visited = true;
    let mut dir = Direction::Up;

    'outer: loop {
        let (mut next, mut bk);
        (next, x, y, bk) = check_delta(dir, x, y, contents);
        if bk {
            break 'outer;
        }

        let mut count = 1;
        while contents[y as usize][x as usize].c == b'#' {
            x -= next.0;
            y -= next.1;

            dir = dir.rotate_right();

            (next, x, y, bk) = check_delta(dir, x, y, contents);
            if bk {
                break 'outer;
            }

            count += 1;

            if count > 4 {
                panic!("trapped??");
            }
        }

        contents[y as usize][x as usize].visited = true;
    }

    for row in &mut *contents {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Position {
                        c: _,
                        visited: true,
                    } => "X".into(),
                    Position { c, visited: false } => String::from_utf8(vec![*c]).unwrap(),
                }
            );
        }
        println!();
    }

    printwriteln!(
        writer,
        "part 1: {}",
        contents.iter().flatten().fold(0, |acc, p| {
            if p.visited {
                acc + 1
            } else {
                acc
            }
        })
    )
    .unwrap();
}

//fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<Vec<Position>>) {}

fn find_start(contents: &[Vec<Position>]) -> (i32, i32) {
    for (y, row) in contents.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.c == b'^' {
                return (x as i32, y as i32);
            }
        }
    }

    panic!("^ not found");
}

fn check_delta(
    dir: Direction,
    mut x: i32,
    mut y: i32,
    contents: &[Vec<Position>],
) -> ((i32, i32), i32, i32, bool) {
    let next = dir.delta();
    x += next.0;
    y += next.1;

    if y < 0 || y >= contents.len() as i32 || x < 0 || x >= contents[0].len() as i32 {
        return (next, x, y, true);
    }

    (next, x, y, false)
}
