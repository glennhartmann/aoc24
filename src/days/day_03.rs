use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

use {once_cell::sync::Lazy, regex::Regex};

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 3).unwrap();

    part1(&mut writer, &contents);
    part2(&mut writer, &contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<&str>) {
    let mut total = 0;
    for line in contents {
        total += find_and_eval_muls_part1(line);
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, contents: &Vec<&str>) {
    let mut total = 0;
    let mut doo = true;
    for line in contents {
        let i;
        (i, doo) = find_and_eval_muls_part2(line, doo);
        total += i;
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

fn find_and_eval_muls_part1(line: &str) -> i32 {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    let caps = RE.captures_iter(line);

    let mut total = 0;
    for cap in caps {
        let i1 = cap[1].parse::<i32>().unwrap();
        let i2 = cap[2].parse::<i32>().unwrap();

        total += i1 * i2;
    }

    total
}

fn find_and_eval_muls_part2(line: &str, mut doo: bool) -> (i32, bool) {
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap());
    let caps = RE.captures_iter(line);

    let mut total = 0;
    for cap in caps {
        match &cap[0] {
            "do()" => doo = true,
            "don't()" => doo = false,
            _ => {
                if doo {
                    let i1 = cap[2].parse::<i32>().unwrap();
                    let i2 = cap[3].parse::<i32>().unwrap();

                    total += i1 * i2;
                }
            }
        }
    }

    (total, doo)
}
