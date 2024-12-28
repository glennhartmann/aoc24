use std::{
    cmp,
    io::{BufWriter, Write},
    ops::{Index, RangeFrom},
};

use aoclib_rs::{dir::Dir4, prep_io, printwriteln};

const NUM_PAD: [(i32, i32); 11] = [
    (1, 3), // 0
    (0, 2), // 1
    (1, 2), // 2
    (2, 2), // 3
    (0, 1), // 4
    (1, 1), // 5
    (2, 1), // 6
    (0, 0), // 7
    (1, 0), // 8
    (2, 0), // 9
    (2, 3), // A
];

const NUM_PAD_BLANK: (i32, i32) = (0, 3);

const DIR_PAD: [(i32, i32); 5] = [
    (1, 0), // ^
    (2, 0), // A
    (0, 1), // <
    (1, 1), // v
    (2, 1), // >
];

const DIR_PAD_BLANK: (i32, i32) = (0, 0);

#[derive(Debug, Copy, Clone)]
enum Command {
    Dir4(Dir4),
    A,
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 21).unwrap();

    part1(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<&str>) {
    let mut total: u64 = 0;
    for line in contents {
        let num_pad_sequences = compute_num_pad_sequences(line, pos_of_num_pad('A'));

        let dir_pad_sequences_1 = compute_dir_pad_sequences(&num_pad_sequences);
        let dir_pad_sequences_1 = filter_to_shortest(&dir_pad_sequences_1);

        let dir_pad_sequence_lens_2 = compute_dir_pad_sequence_lens(&dir_pad_sequences_1);

        let lp = line[..line.len() - 1].parse::<u64>().unwrap();
        total += dir_pad_sequence_lens_2 * lp;
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn compute_num_pad_sequences(suffix: &str, pos: (i32, i32)) -> Vec<Vec<Command>> {
    compute_sequences_rec(
        suffix,
        pos,
        NUM_PAD_BLANK,
        |s| s.is_empty(),
        |s| s.chars().next(),
        pos_of_num_pad,
    )
}

fn pos_of_num_pad(c: char) -> (i32, i32) {
    match c {
        '0' => NUM_PAD[0],
        '1' => NUM_PAD[1],
        '2' => NUM_PAD[2],
        '3' => NUM_PAD[3],
        '4' => NUM_PAD[4],
        '5' => NUM_PAD[5],
        '6' => NUM_PAD[6],
        '7' => NUM_PAD[7],
        '8' => NUM_PAD[8],
        '9' => NUM_PAD[9],
        'A' => NUM_PAD[10],
        _ => panic!("invalid character: {}", c),
    }
}

fn compute_sequences_rec<T, C>(
    suffix: &T,
    pos: (i32, i32),
    blank_space: (i32, i32),
    is_empty: fn(&T) -> bool,
    first: fn(&T) -> Option<C>,
    pos_of_pad: fn(C) -> (i32, i32),
) -> Vec<Vec<Command>>
where
    T: Index<RangeFrom<usize>, Output = T> + ?Sized,
{
    if is_empty(suffix) {
        return vec![Vec::new()];
    }

    let new_pos = pos_of_pad(first(suffix).unwrap());

    cross_product(
        compute_sequences_to_pos(pos, new_pos, blank_space),
        compute_sequences_rec(
            &suffix[1..],
            new_pos,
            blank_space,
            is_empty,
            first,
            pos_of_pad,
        ),
    )
}

fn compute_sequences_to_pos(
    pos: (i32, i32),
    new_pos: (i32, i32),
    blank_space: (i32, i32),
) -> Vec<Vec<Command>> {
    let mut seqs_to_c = compute_sequences_to_pos_rec(pos, new_pos, blank_space);
    for k in seqs_to_c.iter_mut() {
        k.reverse();
    }

    seqs_to_c
}

fn compute_sequences_to_pos_rec(
    pos: (i32, i32),
    new_pos: (i32, i32),
    blank_space: (i32, i32),
) -> Vec<Vec<Command>> {
    if pos == new_pos {
        return vec![vec![Command::A]];
    }

    let pos_diff = diff_pos(new_pos, pos);

    let mut seqs = Vec::new();
    let mut rec = |pos_diff_dimension, next, positive_direction, negative_direction| {
        if pos_diff_dimension != 0 && next != blank_space {
            let mut seqs_inner = compute_sequences_to_pos_rec(next, new_pos, blank_space);
            for seq in seqs_inner.iter_mut() {
                seq.push(if pos_diff_dimension > 0 {
                    positive_direction
                } else {
                    negative_direction
                });
            }
            seqs.append(&mut seqs_inner);
        }
    };

    rec(
        pos_diff.0,
        (pos.0 + sign(pos_diff.0), pos.1),
        Command::Dir4(Dir4::Right),
        Command::Dir4(Dir4::Left),
    );

    rec(
        pos_diff.1,
        (pos.0, (pos.1 + sign(pos_diff.1))),
        Command::Dir4(Dir4::Down),
        Command::Dir4(Dir4::Up),
    );

    seqs
}

fn diff_pos(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn sign(i: i32) -> i32 {
    match i {
        i32::MIN..0 => -1,
        0 => 0,
        1..=i32::MAX => 1,
    }
}

fn cross_product(v1: Vec<Vec<Command>>, v2: Vec<Vec<Command>>) -> Vec<Vec<Command>> {
    let mut r = Vec::with_capacity(v1.len() * v2.len());
    for v1e in v1 {
        for v2e in &v2 {
            r.push(
                vec![v1e.clone(), v2e.clone()]
                    .into_iter()
                    .flatten()
                    .collect(),
            );
        }
    }

    r
}

fn compute_dir_pad_sequences(sequences: &Vec<Vec<Command>>) -> Vec<Vec<Command>> {
    let mut v = Vec::with_capacity(sequences.len());
    for seq in sequences {
        v.push(compute_dir_pad_sequences_internal(
            seq,
            pos_of_dir_pad(Command::A),
        ));
    }

    v.into_iter().flatten().collect()
}

fn compute_dir_pad_sequences_internal(suffix: &[Command], pos: (i32, i32)) -> Vec<Vec<Command>> {
    compute_sequences_rec(
        suffix,
        pos,
        DIR_PAD_BLANK,
        |s| s.is_empty(),
        |s| Some(s[0]),
        pos_of_dir_pad,
    )
}

fn pos_of_dir_pad(c: Command) -> (i32, i32) {
    match c {
        Command::Dir4(Dir4::Up) => DIR_PAD[0],
        Command::Dir4(Dir4::Down) => DIR_PAD[3],
        Command::Dir4(Dir4::Left) => DIR_PAD[2],
        Command::Dir4(Dir4::Right) => DIR_PAD[4],
        Command::A => DIR_PAD[1],
    }
}

fn filter_to_shortest(seqs: &[Vec<Command>]) -> Vec<&Vec<Command>> {
    let min_len = seqs.iter().min_by_key(|seq| seq.len()).unwrap().len();
    seqs.iter().filter(|seq| seq.len() == min_len).collect()
}

fn compute_dir_pad_sequence_lens(sequences: &Vec<&Vec<Command>>) -> u64 {
    let mut v = u64::MAX;
    for seq in sequences {
        v = cmp::min(
            v,
            compute_dir_pad_sequence_lens_rec(seq, pos_of_dir_pad(Command::A)),
        );
    }

    v
}

fn compute_dir_pad_sequence_lens_rec(suffix: &[Command], pos: (i32, i32)) -> u64 {
    if suffix.is_empty() {
        return 0;
    }

    let new_pos = pos_of_dir_pad(suffix[0]);
    let seqs_to_d = compute_sequences_to_pos(pos, new_pos, DIR_PAD_BLANK);

    seqs_to_d.iter().min_by_key(|seq| seq.len()).unwrap().len() as u64
        + compute_dir_pad_sequence_lens_rec(&suffix[1..], new_pos)
}
