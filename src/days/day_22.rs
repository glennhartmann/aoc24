use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln};

const MUL1: u64 = 64;
const MUL2: u64 = 2_048;
const DIV: u64 = 32;
const MOD: u64 = 16_777_216;

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 22).unwrap();
    let contents: Vec<u64> = contents.iter().map(|n| n.parse().unwrap()).collect();

    let (prices, price_changes) = part1(&mut writer, &contents);
    part2(&mut writer, &prices, &price_changes);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &[u64]) -> (Vec<Vec<u64>>, Vec<Vec<i64>>) {
    let mut secrets = contents.to_owned();
    let mut prices = Vec::with_capacity(secrets.len());
    let mut price_changes = Vec::with_capacity(secrets.len());
    for (i, s) in &mut secrets.iter_mut().enumerate() {
        let mut single_prices = Vec::with_capacity(2_000);
        let mut single_price_changes = Vec::with_capacity(2_000);
        for _ in 0..2_000 {
            let prev = *s;
            *s = get_next_secret(*s);

            let price = *s % 10;
            single_prices.push(price);
            single_price_changes.push(price as i64 - (prev % 10) as i64);
        }
        prices.push(single_prices);
        price_changes.push(single_price_changes);
        println!("{}: {}", contents[i], s);
    }

    printwriteln!(writer, "part 1: {}", secrets.iter().sum::<u64>()).unwrap();

    (prices, price_changes)
}

fn part2<W: Write>(writer: &mut BufWriter<W>, prices: &[Vec<u64>], price_changes: &Vec<Vec<i64>>) {
    let mut hs = HashSet::new();
    let mut chunk_indices = Vec::with_capacity(price_changes.len());
    for pc in price_changes {
        let indices = overlapping_chunks_and_chunk_indices(pc);
        for chunk in indices.keys() {
            hs.insert(chunk.clone());
        }
        chunk_indices.push(indices);
    }

    let mut m = 0;
    for (c, chunk) in hs.iter().enumerate() {
        println!(
            "about to get_total_bananas() for chunk {} of {}",
            c,
            hs.len()
        );
        m = max(
            m,
            get_total_bananas(prices, price_changes, chunk, &chunk_indices),
        );
    }

    printwriteln!(writer, "part 2: {}", m).unwrap();
}

fn get_next_secret(mut s: u64) -> u64 {
    s = (s ^ (s * MUL1)) % MOD;
    s = (s ^ (s / DIV)) % MOD;
    s = (s ^ (s * MUL2)) % MOD;

    s
}

fn overlapping_chunks_and_chunk_indices(pc: &[i64]) -> HashMap<Vec<i64>, usize> {
    let mut hm = HashMap::with_capacity(pc.len());
    for i in 0..(pc.len() - 3) {
        let chunk = vec![pc[i], pc[i + 1], pc[i + 2], pc[i + 3]];
        hm.entry(chunk).or_insert(i + 3);
    }

    hm
}

fn get_total_bananas(
    prices: &[Vec<u64>],
    price_changes: &[Vec<i64>],
    chunk: &[i64],
    chunk_indices: &[HashMap<Vec<i64>, usize>],
) -> u64 {
    let mut total = 0;
    for i in 0..price_changes.len() {
        match chunk_indices[i].get(chunk) {
            None => (),
            Some(&index) => {
                total += prices[i][index];
            }
        };
    }

    total
}
