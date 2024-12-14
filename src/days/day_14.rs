use std::io::{BufWriter, Write};

use aoclib_rs::{prep_io, printwriteln, split_and_parse};

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

const PART2_ANSWER: i64 = 6512;

#[derive(Copy, Clone)]
struct Robot {
    point: Point,
    velocity: Velocity,
}

impl Robot {
    fn new(p: Point, v: Velocity) -> Robot {
        Robot {
            point: p,
            velocity: v,
        }
    }

    fn step(&mut self) {
        self.point.x = (self.point.x + self.velocity.x).rem_euclid(WIDTH);
        self.point.y = (self.point.y + self.velocity.y).rem_euclid(HEIGHT);
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}

#[derive(Copy, Clone)]
struct Velocity {
    x: i64,
    y: i64,
}

impl Velocity {
    fn new(x: i64, y: i64) -> Velocity {
        Velocity { x, y }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 14).unwrap();
    let mut contents: Vec<Robot> = contents
        .iter()
        .map(|line| {
            let pv: Vec<&str> = line.strip_prefix("p=").unwrap().split(" v=").collect();
            let p = split_and_parse(pv[0], ",").unwrap();
            let v = split_and_parse(pv[1], ",").unwrap();

            Robot::new(Point::new(p[0], p[1]), Velocity::new(v[0], v[1]))
        })
        .collect();

    part1(&mut writer, &mut contents.clone());
    part2(&mut writer, &mut contents);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, robots: &mut Vec<Robot>) {
    for _ in 0..100 {
        for robot in &mut *robots {
            robot.step();
        }
    }

    let mut upper_left = 0;
    let mut upper_right = 0;
    let mut lower_left = 0;
    let mut lower_right = 0;
    for robot in robots {
        if robot.point.x < WIDTH / 2 && robot.point.y < HEIGHT / 2 {
            upper_left += 1;
        } else if robot.point.x < WIDTH / 2 && robot.point.y > HEIGHT / 2 {
            lower_left += 1;
        } else if robot.point.x > WIDTH / 2 && robot.point.y < HEIGHT / 2 {
            upper_right += 1;
        } else if robot.point.x > WIDTH / 2 && robot.point.y > HEIGHT / 2 {
            lower_right += 1;
        }
    }

    println!(
        "upper_left: {}; upper_right: {}; lower_left: {}; lower_right: {}",
        upper_left, upper_right, lower_left, lower_right
    );
    printwriteln!(
        writer,
        "part 1: {}",
        upper_left * upper_right * lower_left * lower_right
    )
    .unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, robots: &mut Vec<Robot>) {
    for _ in 0..PART2_ANSWER {
        for robot in &mut *robots {
            robot.step();
        }
    }

    let mut grid: Vec<Vec<u8>> = vec![vec![b'.'; WIDTH as usize]; HEIGHT as usize];
    for robot in robots.iter() {
        grid[robot.point.y as usize][robot.point.x as usize] = b'R';
    }

    for row in grid.iter() {
        println!("{}", String::from_utf8(row.clone()).unwrap());
    }

    printwriteln!(writer, "part 2: {}", PART2_ANSWER).unwrap();
}
