use std::io::{BufWriter, Write};

use aoclib_rs::{
    dir::{Dir4, Direction},
    prep_io, printwriteln, u8_to_string,
};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 15).unwrap();

    let mut map = Vec::new();
    let mut line = 0;
    while !contents[line].is_empty() {
        map.push(contents[line].as_bytes().to_owned());
        line += 1;
    }

    let mut map2 = Vec::with_capacity(map.len());
    for line in &map {
        let mut nl = Vec::with_capacity(line.len() * 2);
        for c in line {
            nl.append(&mut match c {
                b'#' => vec![b'#', b'#'],
                b'O' => vec![b'[', b']'],
                b'.' => vec![b'.', b'.'],
                b'@' => vec![b'@', b'.'],
                _ => panic!("invalid map character: {}", u8_to_string(*c)),
            });
        }

        map2.push(nl);
    }

    let mut dirs = Vec::new();
    for ln in contents[(line + 1)..].iter() {
        let mut v: Vec<Dir4> = ln
            .as_bytes()
            .iter()
            .map(|c| match c {
                b'^' => Dir4::Up,
                b'v' => Dir4::Down,
                b'<' => Dir4::Left,
                b'>' => Dir4::Right,
                _ => panic!("invalid direction: {}", u8_to_string(*c)),
            })
            .collect();
        dirs.append(&mut v)
    }

    let initial_pos = find_robot(&map);
    part1(&mut writer, &mut map, initial_pos, &dirs);

    let initial_pos = find_robot(&map2);
    part2(&mut writer, &mut map2, initial_pos, &dirs);
}

fn part1<W: Write>(
    writer: &mut BufWriter<W>,
    map: &mut Vec<Vec<u8>>,
    mut pos: (usize, usize),
    dirs: &Vec<Dir4>,
) {
    for dir in dirs {
        (_, pos) = attempt_move_1(map, pos, *dir);
    }

    println!("map:\n{}", map_to_string(map));

    let mut total = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'O' {
                total += x + 100 * y;
            }
        }
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    map: &mut Vec<Vec<u8>>,
    mut pos: (usize, usize),
    dirs: &Vec<Dir4>,
) {
    println!("initial map:\n{}", map_to_string(map));

    for dir in dirs {
        if can_move_2(map, pos, *dir) {
            pos = do_move_2(map, pos, *dir);
        }
    }

    println!("final map:\n{}", map_to_string(map));

    let mut total = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'[' {
                total += x + 100 * y;
            }
        }
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

fn find_robot(map: &[Vec<u8>]) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == b'@' {
                return (x, y);
            }
        }
    }

    panic!("robot not found");
}

fn attempt_move_1(
    map: &mut Vec<Vec<u8>>,
    pos: (usize, usize),
    dir: Dir4,
) -> (bool, (usize, usize)) {
    let (next_x, next_y) = dir.apply_delta_to_usizes(pos);

    let next = map[next_y][next_x];
    match next {
        b'#' => (false, (pos.0, pos.1)),
        b'.' => {
            map[next_y][next_x] = map[pos.1][pos.0];
            map[pos.1][pos.0] = b'.';

            (true, (next_x, next_y))
        }
        b'O' => {
            let mut next = pos;
            let (success, _) = attempt_move_1(map, (next_x, next_y), dir);
            if success {
                map[next_y][next_x] = map[pos.1][pos.0];
                map[pos.1][pos.0] = b'.';
                next = (next_x, next_y);
            }

            (success, next)
        }
        _ => panic!("invalid map character: {}", u8_to_string(next)),
    }
}

fn can_move_2(map: &Vec<Vec<u8>>, pos: (usize, usize), dir: Dir4) -> bool {
    let (next_x, next_y) = dir.apply_delta_to_usizes(pos);

    let next = map[next_y][next_x];
    match next {
        b'#' => false,
        b'.' => true,
        b'[' | b']' => {
            can_move_2(map, (next_x, next_y), dir)
                && if dir == Dir4::Up || dir == Dir4::Down {
                    can_move_2(
                        map,
                        (if next == b'[' { next_x + 1 } else { next_x - 1 }, next_y),
                        dir,
                    )
                } else {
                    true
                }
        }
        _ => panic!("invalid map character: {}", u8_to_string(next)),
    }
}

fn do_move_2(map: &mut Vec<Vec<u8>>, pos: (usize, usize), dir: Dir4) -> (usize, usize) {
    let (next_x, next_y) = dir.apply_delta_to_usizes(pos);

    let next = map[next_y][next_x];
    match next {
        b'#' => panic!("do_move_2() called on unmovable input"),
        b'.' => {
            map[next_y][next_x] = map[pos.1][pos.0];
            map[pos.1][pos.0] = b'.';
        }
        b'[' | b']' => {
            do_move_2(map, (next_x, next_y), dir);
            if dir == Dir4::Up || dir == Dir4::Down {
                do_move_2(
                    map,
                    (if next == b'[' { next_x + 1 } else { next_x - 1 }, next_y),
                    dir,
                );
            }

            map[next_y][next_x] = map[pos.1][pos.0];
            map[pos.1][pos.0] = b'.';
        }
        _ => panic!("invalid map character: {}", u8_to_string(next)),
    };

    (next_x, next_y)
}

fn map_to_string(map: &Vec<Vec<u8>>) -> String {
    let mut s = String::new();
    for line in map {
        s.push_str(&String::from_utf8(line.clone()).unwrap());
        s.push('\n');
    }

    s
}
