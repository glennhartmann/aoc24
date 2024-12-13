use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 7).unwrap();
    let contents: Vec<Vec<&str>> = contents.iter().map(|s| s.split(": ").collect()).collect();

    let mut targets: Vec<u64> = Vec::new();
    let mut operands: Vec<Vec<u64>> = Vec::new();

    for line in contents {
        targets.push(line[0].parse().unwrap());
        operands.push(split_and_parse(line[1], " ").unwrap());
    }

    part1(&mut writer, &targets, &operands);
    part2(&mut writer, &targets, &operands);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, targets: &[u64], operands: &[Vec<u64>]) {
    let s = solve(targets, operands, false);
    printwriteln!(writer, "part 1: {}", s).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, targets: &[u64], operands: &[Vec<u64>]) {
    let s = solve(targets, operands, true);
    printwriteln!(writer, "part 2: {}", s).unwrap();
}

fn solve(targets: &[u64], operands: &[Vec<u64>], part2: bool) -> u64 {
    let mut total = 0;
    for (i, ops) in operands.iter().enumerate() {
        let target = targets[i];
        let mut operators = Vec::with_capacity(ops.len() - 1);
        if permute_recursive(target, &ops[1..], ops[0], &mut operators, part2) == target {
            total += target;
            print_formula(target, ops, &mut operators);
        } else {
            println!("{} != {:?}", target, ops);
        }
    }

    total
}

fn permute_recursive(
    target: u64,
    ops: &[u64],
    total: u64,
    operators: &mut Vec<&str>,
    part2: bool,
) -> u64 {
    if total > target {
        return 0;
    }

    if ops.is_empty() {
        return if total == target { total } else { 0 };
    }

    let t = permute_recursive(target, &ops[1..], total * ops[0], operators, part2);
    if t == target {
        //println!("{} * {} = {}", total, ops[0], total * ops[0]);
        operators.push("*");
        return t;
    }
    let t = permute_recursive(target, &ops[1..], total + ops[0], operators, part2);
    if t == target {
        //println!("{} + {} = {}", total, ops[0], total + ops[0]);
        operators.push("+");
        return t;
    }

    if part2 {
        let total2 = (total.to_string() + ops[0].to_string().as_str())
            .parse()
            .unwrap();
        let t = permute_recursive(target, &ops[1..], total2, operators, part2);
        if t == target {
            //println!("{} || {} = {}", total, ops[0], total2);
            operators.push("||");
            return t;
        }
    }

    0
}

fn print_formula(target: u64, ops: &Vec<u64>, operators: &mut Vec<&str>) {
    print!("{} = ", target);

    let mut first = true;
    for op in ops {
        if !first {
            print!(" {} ", operators.pop().unwrap());
        }
        first = false;

        print!("{}", op);
    }

    println!();
}
