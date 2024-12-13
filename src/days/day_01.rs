use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 1).unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in contents {
        let lsp = split_and_parse(line, "   ").unwrap();
        left.push(lsp[0]);
        right.push(lsp[1]);
    }

    left.sort();
    right.sort();

    part1(&mut writer, &left, &right);
    part2(&mut writer, &left, &right);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, left: &[i32], right: &[i32]) {
    let mut total = 0;
    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, left: &Vec<i32>, right: &Vec<i32>) {
    let mut rightm = HashMap::new();
    for v in right {
        rightm.entry(v).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut total = 0;
    for v in left {
        total += *v * *rightm.entry(v).or_insert(0);
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}
