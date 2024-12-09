use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use aoclib_rs::printwriteln;

pub fn run() {
    let write_file = File::create("outputs/04.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/04.txt").unwrap();
    let contents: Vec<&[u8]> = contents.trim().split('\n').map(|s| s.as_bytes()).collect();
    let contents = pad(&contents);

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
    for i in 4..(contents.len() - 4) {
        for j in 4..(contents[i].len() - 4) {
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
    for i in 4..(contents.len() - 4) {
        for j in 4..(contents[i].len() - 4) {
            if contents[i][j] != b'X' {
                continue;
            }
            if contents[(i as i32 + horiz) as usize][(j as i32 + vert) as usize] != b'M' {
                continue;
            }
            if contents[(i as i32 + 2 * horiz) as usize][(j as i32 + 2 * vert) as usize] != b'A' {
                continue;
            }
            if contents[(i as i32 + 3 * horiz) as usize][(j as i32 + 3 * vert) as usize] != b'S' {
                continue;
            }
            total += 1
        }
    }

    total
}

fn pad(contents: &Vec<&[u8]>) -> Vec<Vec<u8>> {
    let mut r = Vec::with_capacity(contents.len());
    let mut prefix = vec![vec![b' '; contents[0].len() + 8]; 4];
    r.append(&mut prefix);

    for line in contents {
        let mut v = Vec::with_capacity(line.len() + 8);
        let mut prefix = vec![b' '; 4];
        let mut middle = line.to_vec();
        let mut suffix = vec![b' '; 4];
        v.append(&mut prefix);
        v.append(&mut middle);
        v.append(&mut suffix);

        r.push(v);
    }

    let mut suffix = vec![vec![b' '; contents[0].len() + 8]; 4];
    r.append(&mut suffix);

    r
}
