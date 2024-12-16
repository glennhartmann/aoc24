use std::{
    collections::HashSet,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln, u8_to_string, Direction};

#[derive(Copy, Clone)]
struct Position {
    c: u8,
    visited: bool,
}

impl Position {
    fn new(c: u8) -> Position {
        Position { c, visited: false }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 6).unwrap();
    let contents: Vec<Vec<Position>> = contents
        .iter()
        .map(|s| s.as_bytes().iter().map(|b| Position::new(*b)).collect())
        .collect();

    let part1_contents = part1(&mut writer, contents.clone());
    part2(&mut writer, &contents, &part1_contents);
}

fn part1<W: Write>(
    writer: &mut BufWriter<W>,
    mut contents: Vec<Vec<Position>>,
) -> Vec<Vec<Position>> {
    let start = find_start(&contents);
    check_for_loop(&mut contents, start);

    for row in &mut *contents {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Position {
                        c: _,
                        visited: true,
                    } => "X".into(),
                    Position { c, visited: false } => u8_to_string(*c),
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

    contents
}

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    contents: &[Vec<Position>],
    part1_contents: &[Vec<Position>],
) {
    let start = find_start(contents);

    let mut potential_locations = Vec::new();
    for (y, row) in part1_contents.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.visited && (x as i32 != start.0 || y as i32 != start.1) {
                potential_locations.push((x, y));
            }
        }
    }

    let mut total = 0;
    for (i, pos) in potential_locations.iter().enumerate() {
        let mut contents = contents.to_owned();
        contents[pos.1][pos.0].c = b'#';

        if check_for_loop(&mut contents, start) {
            total += 1;
        }

        println!(
            "done another loop ({}/{}) - total so far {}",
            i,
            potential_locations.len(),
            total
        );
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

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

fn check_for_loop(contents: &mut [Vec<Position>], start: (i32, i32)) -> bool {
    let (mut x, mut y) = start;
    let mut dir = Direction::Up;

    contents[y as usize][x as usize].visited = true;
    let mut hs = HashSet::new();
    hs.insert((start, dir));

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

        if hs.contains(&((x, y), dir)) {
            return true;
        }

        contents[y as usize][x as usize].visited = true;
        hs.insert(((x, y), dir));
    }

    false
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
