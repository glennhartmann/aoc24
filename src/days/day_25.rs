use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

type Lock = [u8; 5];
type Key = [u8; 5];

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 25).unwrap();

    let mut locks: Vec<Lock> = Vec::new();
    let mut keys: Vec<Key> = Vec::new();
    for entry in contents.chunks(8) {
        let lock_mode = entry[0].starts_with('#');
        let mut e: [u8; 5] = [0; 5];
        for (i, line) in entry[..7].iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match lock_mode {
                    true => {
                        if c == '#' {
                            e[j] = i as u8;
                        }
                    }
                    false => {
                        if c == '.' {
                            e[j] = (5 - i) as u8;
                        }
                    }
                };
            }
        }

        match lock_mode {
            true => {
                locks.push(e);
            }
            false => {
                keys.push(e);
            }
        };
    }

    println!("locks: {:?}", locks);
    println!("keys: {:?}", keys);

    part1(&mut writer, &locks, &keys);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, locks: &Vec<Lock>, keys: &Vec<Key>) {
    let mut total = 0;
    for lock in locks {
        for key in keys {
            if key_can_fit_lock(*lock, *key) {
                total += 1;
            }
        }
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn key_can_fit_lock(lock: Lock, key: Key) -> bool {
    for pin in 0..lock.len() {
        if lock[pin] + key[pin] > 5 {
            return false;
        }
    }

    true
}
