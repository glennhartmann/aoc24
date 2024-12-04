use std::{
    fmt,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use crate::common::printwriteln;

#[derive(PartialEq, Copy, Clone)]
enum SpanType {
    File,
    Space,
}

#[derive(Copy, Clone)]
struct Span {
    t: SpanType,
    id: i64,
    len: usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.len {
            if self.t == SpanType::File {
                write!(f, "{}", self.id)?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

pub fn run() {
    let write_file = File::create("outputs/09.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/09.txt").unwrap();
    let contents: Vec<usize> = contents
        .trim()
        .split("")
        .filter_map(|n| {
            if n.is_empty() {
                None
            } else {
                Some(n.parse::<usize>().unwrap())
            }
        })
        .collect();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &[usize]) {
    let mut disk: Vec<i64> = Vec::with_capacity(contents.iter().sum());
    let mut i = 0;
    let mut id = 0;
    while i < contents.len() {
        for _ in 0..contents[i] {
            disk.push(id);
        }

        if i == contents.len() - 1 {
            break;
        }

        for _ in 0..contents[i + 1] {
            disk.push(-1);
        }

        i += 2;
        id += 1;
    }

    loop {
        let item = disk.pop().unwrap();
        if item == -1 {
            continue;
        }

        let first_space = match disk.iter().position(|&x| x == -1) {
            Some(i) => i,
            None => {
                disk.push(item);
                break;
            }
        };

        disk[first_space] = item;
    }

    let mut total = 0;
    for (i, id) in disk.iter().enumerate() {
        total += (i as i64) * id;
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[usize]) {
    let mut disk = Vec::with_capacity(contents.len());
    let mut i = 0;
    let mut id = 0;
    while i < contents.len() {
        disk.push(Span {
            t: SpanType::File,
            id,
            len: contents[i],
        });

        if i == contents.len() - 1 {
            break;
        }

        disk.push(Span {
            t: SpanType::Space,
            id: -1,
            len: contents[i + 1],
        });

        i += 2;
        id += 1;
    }

    let mut i = disk.len();
    loop {
        i = match i.checked_sub(1) {
            Some(j) => j,
            None => break,
        };

        let span = disk[i];

        if span.t == SpanType::Space {
            continue;
        }

        let first_big_enough_space = match disk
            .iter()
            .position(|x| x.t == SpanType::Space && x.len >= span.len)
        {
            Some(i) => i,
            None => continue,
        };

        if first_big_enough_space > i {
            continue;
        }

        let ns_len = disk[first_big_enough_space].len - span.len;
        if ns_len > 0 {
            disk.insert(
                first_big_enough_space + 1,
                Span {
                    t: SpanType::Space,
                    id: -1,
                    len: ns_len,
                },
            );
            i += 1;
        }
        disk[first_big_enough_space] = span;
        disk[i].t = SpanType::Space;
    }

    let mut total = 0;
    let mut i: usize = 0;
    for span in disk {
        if span.t == SpanType::File {
            for j in i..(i + span.len) {
                total += (j as i64) * span.id;
            }
        }

        i += span.len;
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}
