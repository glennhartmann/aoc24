use std::{collections::HashMap, io::Write};

use aoclib_rs::{prep_io, printwriteln, trie::Trie};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 19).unwrap();

    let patterns: Vec<_> = contents[0].split(", ").collect();
    let designs = &contents[2..];

    let mut trie = Trie::new();
    for p in &patterns {
        trie.insert(p);
    }

    let mut hm = HashMap::new();
    let mut possible_patterns = 0;
    let mut possible_ways = 0;
    for d in designs {
        let n = count_possibilities_rec(&trie, d, &mut hm);
        if n > 0 {
            println!("{} is possible in {} ways", d, n);
            possible_patterns += 1;
            possible_ways += n;
        } else {
            println!("{} is not possible", d);
        }
    }

    printwriteln!(writer, "part 1: {}", possible_patterns).unwrap();
    printwriteln!(writer, "part 2: {}", possible_ways).unwrap();
}

fn count_possibilities_rec(trie: &Trie, pattern: &str, hm: &mut HashMap<String, u64>) -> u64 {
    match hm.get(pattern) {
        None => (),
        Some(&u) => return u,
    };

    let mut total = 0;
    for i in 1..=pattern.len() {
        match trie.find(&pattern[..i]) {
            None => {
                hm.insert(pattern.to_owned(), total);
                return total;
            }
            Some(sub_trie) => {
                if !sub_trie.is_terminal() {
                    continue;
                } else if i == pattern.len() {
                    total += 1;
                } else {
                    total += count_possibilities_rec(trie, &pattern[i..], hm);
                }
            }
        };
    }

    hm.insert(pattern.to_owned(), total);
    total
}
