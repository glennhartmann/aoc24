use std::io::{BufWriter, Write};

use aoclib_rs::{pad, prep_io, printwriteln, usize_plus_i};

const PADDING: usize = 4;

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 4).unwrap();
    let contents = contents.iter().map(|s| s.as_bytes()).collect();
    let contents = pad(&contents, PADDING, b' ');

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<u8>]) {
    // horizontal
    let mut total = count_part1(contents, 1, 0);
    total += count_part1(contents, -1, 0);

    // vertical
    total += count_part1(contents, 0, 1);
    total += count_part1(contents, 0, -1);

    // diagonal
    total += count_part1(contents, 1, 1);
    total += count_part1(contents, 1, -1);
    total += count_part1(contents, -1, 1);
    total += count_part1(contents, -1, -1);

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &[Vec<u8>]) {
    let mut total = 0;
    for i in PADDING..(contents.len() - PADDING) {
        for j in PADDING..(contents[i].len() - PADDING) {
            let mas1 = (contents[i][j] == b'M'
                && contents[i + 1][j + 1] == b'A'
                && contents[i + 2][j + 2] == b'S')
                || (contents[i][j] == b'S'
                    && contents[i + 1][j + 1] == b'A'
                    && contents[i + 2][j + 2] == b'M');
            let mas2 = (contents[i][j + 2] == b'M'
                && contents[i + 1][j + 1] == b'A'
                && contents[i + 2][j] == b'S')
                || (contents[i][j + 2] == b'S'
                    && contents[i + 1][j + 1] == b'A'
                    && contents[i + 2][j] == b'M');
            if mas1 && mas2 {
                total += 1;
            }
        }
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

fn count_part1(contents: &[Vec<u8>], horiz: i32, vert: i32) -> u32 {
    let mut total = 0;
    for i in PADDING..(contents.len() - PADDING) {
        for j in PADDING..(contents[i].len() - PADDING) {
            if contents[i][j] != b'X' {
                continue;
            }
            if contents[usize_plus_i(i, horiz)][usize_plus_i(j, vert)] != b'M' {
                continue;
            }
            if contents[usize_plus_i(i, 2 * horiz)][usize_plus_i(j, 2 * vert)] != b'A' {
                continue;
            }
            if contents[usize_plus_i(i, 3 * horiz)][usize_plus_i(j, 3 * vert)] != b'S' {
                continue;
            }
            total += 1
        }
    }

    total
}
