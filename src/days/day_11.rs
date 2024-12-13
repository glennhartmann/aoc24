use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use aoclib_rs::{printwriteln, split_and_parse};

pub fn run() {
    let write_file = File::create("outputs/11.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/11.txt").unwrap();
    let contents: Vec<u64> = split_and_parse(contents.trim(), " ").unwrap();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<u64>) {
    let mut m: HashMap<(u64, u64), u64> = HashMap::new();
    let t = compute_recursive(&mut m, contents, 25, 0);
    printwriteln!(writer, "part 1: {}", t).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<u64>) {
    let mut m: HashMap<(u64, u64), u64> = HashMap::new();
    let t = compute_recursive(&mut m, contents, 75, 0);
    printwriteln!(writer, "part 2: {}", t).unwrap();
}

fn compute_recursive(
    m: &mut HashMap<(u64, u64), u64>,
    contents: &Vec<u64>,
    max_lvl: u64,
    lvl: u64,
) -> u64 {
    if lvl == max_lvl {
        return contents.len() as u64;
    }

    let mut t = 0;
    for n in contents {
        match m.get(&(*n, lvl)) {
            Some(v) => t += v,
            None => {
                let cv = compute_n(m, *n, max_lvl, lvl);
                t += cv;
                m.insert((*n, lvl), cv);
            }
        }
    }

    t
}

fn compute_n(m: &mut HashMap<(u64, u64), u64>, n: u64, max_lvl: u64, lvl: u64) -> u64 {
    let mut contents = vec![n];
    let mut new_contents: Vec<u64> = Vec::with_capacity(contents.len() / 2);
    for n in contents.iter_mut() {
        let ns = n.to_string();
        if *n == 0 {
            *n = 1;
        } else if ns.len() % 2 == 0 {
            *n = ns[..ns.len() / 2].parse().unwrap();
            new_contents.push(ns[ns.len() / 2..].parse().unwrap());
        } else {
            *n *= 2024;
        }
    }
    contents.append(&mut new_contents);

    compute_recursive(m, &contents, max_lvl, lvl + 1)
}
