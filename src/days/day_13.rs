use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln};

/*
   Button A: X+c1, Y+d1
   Button B: X+c2, Y+d2
   Prize: X=a, Y=b

   variables v1 (times you press button A), v2 (times you press button B):
   a = c1 * v1 + c2 * v2
   b = d1 * v1 + d2 * v2

   v1 = (a - c2 * v2) / c1, where c1 != 0
   v1 = a / c1 - (c2 / c1) * v2, where c1 != 0

   b = d1 * (a / c1 - (c2 / c1) * v2) + d2 * v2, where c1 != 0
   b = d1 * a / c1 - d1 * (c2 / c1) * v2 + d2 * v2, where c1 != 0
   b = d1 * a / c1 + (-d1 * (c2 / c1) + d2) * v2, where c1 != 0
   v2 = (b - d1 * a / c1) / (d2 - d1 * c2 / c1), where c1 != 0
*/

struct Prize {
    btn_a: Button,
    btn_b: Button,
    prz: PrizeCoords,
}

impl Prize {
    fn new(input: &[&str]) -> Prize {
        let i1 = input[0].strip_prefix("Button A: X+").unwrap();
        let i2 = input[1].strip_prefix("Button B: X+").unwrap();
        let i3 = input[2].strip_prefix("Prize: X=").unwrap();

        let mut i1s = i1.split(", Y+");
        let mut i2s = i2.split(", Y+");
        let mut i3s = i3.split(", Y=");

        let c1 = i1s.next().unwrap().parse().unwrap();
        let d1 = i1s.next().unwrap().parse().unwrap();

        let c2 = i2s.next().unwrap().parse().unwrap();
        let d2 = i2s.next().unwrap().parse().unwrap();

        let a = i3s.next().unwrap().parse().unwrap();
        let b = i3s.next().unwrap().parse().unwrap();

        Prize {
            btn_a: Button { c: c1, d: d1 },
            btn_b: Button { c: c2, d: d2 },
            prz: PrizeCoords { a, b },
        }
    }
}

struct Button {
    c: f64,
    d: f64,
}

struct PrizeCoords {
    a: f64,
    b: f64,
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 13).unwrap();
    let mut contents: &[&str] = &contents;

    let mut prizes = Vec::new();
    loop {
        prizes.push(Prize::new(&contents[..3]));

        if contents.len() > 4 {
            contents = &contents[4..];
        } else {
            break;
        }
    }

    part1(&mut writer, &prizes);
    part2(&mut writer, &mut prizes);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, prizes: &[Prize]) {
    printwriteln!(writer, "part 1: {}", compute_button_presses_total(prizes)).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, prizes: &mut [Prize]) {
    for prize in prizes.iter_mut() {
        prize.prz.a += 10_000_000_000_000.0;
        prize.prz.b += 10_000_000_000_000.0;
    }
    printwriteln!(writer, "part 2: {}", compute_button_presses_total(prizes)).unwrap();
}

fn compute_button_presses_total(prizes: &[Prize]) -> i64 {
    let mut total = 0;
    for prize in prizes {
        let (a, b, c1, d1, c2, d2) = (
            prize.prz.a,
            prize.prz.b,
            prize.btn_a.c,
            prize.btn_a.d,
            prize.btn_b.c,
            prize.btn_b.d,
        );
        let v2 = (b - d1 * a / c1) / (d2 - d1 * c2 / c1);
        let v1 = a / c1 - (c2 / c1) * v2;

        if is_probably_int(v1) && is_probably_int(v2) && v1 >= 0.0 && v2 >= 0.0 {
            total += v1.round() as i64 * 3 + v2.round() as i64;
        }
    }

    total
}

fn is_probably_int(f: f64) -> bool {
    f.fract().abs() < 0.001 || f.fract().abs() > 0.999
}
