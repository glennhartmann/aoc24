use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use crate::common::printwriteln;

pub fn run() {
    let write_file = File::create("outputs/01.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/01.txt").unwrap();
    let contents = contents.split('\n');

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in contents {
        if line.is_empty() {
            continue;
        }

        let mut lsp = line.split("   ");
        left.push(lsp.next().unwrap().parse::<i32>().unwrap());
        right.push(lsp.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    part1(&mut writer, &left, &right);
    part2(&mut writer, &left, &right);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, left: &[i32], right: &[i32]) {
    let mut total = 0;
    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, left: &Vec<i32>, right: &Vec<i32>) {
    let mut rightm = HashMap::new();
    for v in right {
        rightm.insert(
            v,
            match rightm.get(&v) {
                Some(c) => c + 1,
                None => 1,
            },
        );
    }

    let mut total = 0;
    for v in left {
        total += v * match rightm.get(&v) {
            Some(c) => *c,
            None => 0,
        };
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}
