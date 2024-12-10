use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use aoclib_rs::printwriteln;

#[derive(Copy, Clone)]
struct Point {
    val: i8,
    visited: bool,
}

impl Point {
    fn new(i: i8) -> Point {
        Point {
            val: i,
            visited: false,
        }
    }
}

pub fn run() {
    let write_file = File::create("outputs/10.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/10.txt").unwrap();
    let contents: Vec<Vec<Point>> = contents
        .trim()
        .split('\n')
        .map(|r| {
            r.split("")
                .filter_map(|n| {
                    if n.is_empty() {
                        None
                    } else {
                        Some(Point::new(n.parse().unwrap()))
                    }
                })
                .collect()
        })
        .collect();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<Point>]) {
    printwriteln!(
        writer,
        "part 1: {}",
        find_trail_head_and_search(contents, true)
    )
    .unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<Point>]) {
    printwriteln!(
        writer,
        "part 2: {}",
        find_trail_head_and_search(contents, false)
    )
    .unwrap();
}

fn find_trail_head_and_search(contents: &[Vec<Point>], skip_visited: bool) -> u32 {
    let mut count = 0;
    for y in 0..contents.len() {
        for x in 0..contents[y].len() {
            if contents[y][x].val == 0 {
                count += search(
                    &mut contents.to_owned(),
                    x as i32,
                    y as i32,
                    -1,
                    skip_visited,
                );
            }
        }
    }

    count
}

fn search(contents: &mut Vec<Vec<Point>>, x: i32, y: i32, prev: i8, skip_visited: bool) -> u32 {
    if y < 0 || y >= contents.len() as i32 || x < 0 || x >= contents[0].len() as i32 {
        return 0;
    }

    let curr_val;
    {
        let curr = &mut contents[y as usize][x as usize];
        if skip_visited && curr.visited {
            return 0;
        }

        if curr.val != prev + 1 {
            return 0;
        }
        curr.visited = true;

        if curr.val == 9 {
            return 1;
        }

        curr_val = curr.val;
    }

    let mut count = 0;
    count += search(contents, x + 1, y, curr_val, skip_visited);
    count += search(contents, x - 1, y, curr_val, skip_visited);
    count += search(contents, x, y + 1, curr_val, skip_visited);
    count += search(contents, x, y - 1, curr_val, skip_visited);

    count
}
