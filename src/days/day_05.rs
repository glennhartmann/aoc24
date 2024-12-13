use std::{
    collections::HashMap,
    io::{BufWriter, Write},
};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 5).unwrap();

    let mut rules = HashMap::new();
    let mut order = Vec::new();

    let mut rules_mode = true;
    for line in contents {
        if rules_mode {
            if line.is_empty() {
                rules_mode = false;
                continue;
            }

            let lsp = split_and_parse(line, "|").unwrap();
            let entry: &mut Vec<i32> = rules.entry(lsp[1]).or_default();
            entry.push(lsp[0]);

            continue;
        }

        let lsp = split_and_parse(line, ",").unwrap();
        order.push(lsp);
    }

    println!("rules: {:?}", rules);
    println!("order: {:?}", order);

    let mut incorrect_order = part1(&mut writer, &mut rules, &order);
    println!("incorrectly ordered: {:?}", incorrect_order);
    part2(&mut writer, &rules, &mut incorrect_order);
}

fn part1<W: Write>(
    writer: &mut BufWriter<W>,
    rules: &mut HashMap<i32, Vec<i32>>,
    order: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut incorrect_updates = Vec::new();

    let mut total = 0;
    'outer: for update in order {
        println!("update {:?}", update);
        let mut printed = HashMap::new();
        for page in update {
            println!("printing page {}", *page);
            printed.insert(*page, true);

            let rule = rules.entry(*page).or_default();
            for dep in rule {
                let dep_printed = printed.entry(*dep).or_insert(false);
                if *dep_printed {
                    println!("rule {}|{} satisfied", *dep, *page);
                } else if !update.contains(dep) {
                    println!("rule {}|{} not enforced", *dep, *page);
                } else {
                    println!("rule {}|{} not satisfied - fail", *dep, *page);
                    incorrect_updates.push(update.clone());
                    continue 'outer;
                }
            }
        }

        println!("update valid!");
        total += update[update.len() / 2];
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();

    incorrect_updates
}

fn part2<W: Write>(
    writer: &mut BufWriter<W>,
    rules: &HashMap<i32, Vec<i32>>,
    order: &mut Vec<Vec<i32>>,
) {
    let mut total = 0;
    for update in order {
        println!("update {:?}", update);
        let mut new_update = Vec::new();
        let mut printed = HashMap::new();
        while !update.is_empty() {
            fill_in_next(rules, update, &mut new_update, &mut printed);
        }

        println!("new_update: {:?}", new_update);

        total += new_update[new_update.len() / 2];
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

fn fill_in_next(
    rules: &HashMap<i32, Vec<i32>>,
    update: &mut Vec<i32>,
    new_update: &mut Vec<i32>,
    printed: &mut HashMap<i32, bool>,
) {
    for i in 0..update.len() {
        if deps_are_satisfied(rules, update[i], printed, update) {
            println!("deps are satisfied for {}", update[i]);
            new_update.push(update[i]);
            printed.insert(update[i], true);
            update.remove(i);
            break;
        }
    }
}

fn deps_are_satisfied(
    rules: &HashMap<i32, Vec<i32>>,
    i: i32,
    printed: &mut HashMap<i32, bool>,
    update: &[i32],
) -> bool {
    for dep in rules.get(&i).unwrap() {
        let dep_printed = printed.entry(*dep).or_insert(false);
        if !update.contains(dep) {
            continue;
        }
        if !*dep_printed {
            return false;
        }
    }

    true
}
