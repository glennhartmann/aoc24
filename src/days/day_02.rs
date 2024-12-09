use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
};

use aoclib_rs::printwriteln;

pub fn run() {
    let write_file = File::create("outputs/02.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);

    let contents = read_to_string("inputs/02.txt").unwrap();
    let contents = contents.split('\n');
    let contents: Vec<&str> = contents.filter(|line| !line.is_empty()).collect();

    let mut reports = Vec::with_capacity(contents.len());
    for line in contents {
        reports.push(
            line.split(' ')
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    part1(&mut writer, &reports);
    part2(&mut writer, &reports);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, reports: &Vec<Vec<u8>>) {
    let mut safe = 0;
    for report in reports {
        if is_safe_as_is(report) {
            safe += 1;
        }
    }

    printwriteln!(writer, "part 1: {}", safe).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, reports: &Vec<Vec<u8>>) {
    let mut safe = 0;
    for report in reports {
        if is_safe_part2(report) {
            safe += 1;
        }
    }

    printwriteln!(writer, "part 2: {}", safe).unwrap();
}

fn is_safe_as_is(report: &[u8]) -> bool {
    let mut increasing = false;
    for i in 1..report.len() {
        if i == 1 && report[i] > report[i - 1] {
            increasing = true;
        }

        let increasing_and_safe =
            increasing && report[i] > report[i - 1] && report[i] - report[i - 1] <= 3;
        let decreasing_and_safe = !increasing
            && report[i] < report[i - 1]
            && report[i - 1] - report[i] <= 3
            && report[i] != report[i - 1];
        if !increasing_and_safe && !decreasing_and_safe {
            return false;
        }
    }

    true
}

fn is_safe_part2(report: &[u8]) -> bool {
    if is_safe_as_is(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report = report.to_owned();
        report.remove(i);

        if is_safe_as_is(&report) {
            return true;
        }
    }

    false
}
