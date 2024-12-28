use std::io::{BufWriter, Write};

use aoclib_rs::{
    dir::{Dir4, Direction},
    pad, prep_io, printwriteln, u8_to_string,
};

struct Region {
    plant: u8,
    area: u64,
    perimiter: u64,
    sides: u64,
}

impl Region {
    fn new(plant: u8) -> Region {
        Region {
            plant,
            area: 0,
            perimiter: 0,
            sides: 0,
        }
    }
}

#[derive(Copy, Clone)]
struct SidesCounted {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl SidesCounted {
    fn new() -> SidesCounted {
        SidesCounted {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

pub fn run() {
    let mut contents = String::new();
    let (mut writer, contents) = prep_io(&mut contents, 12).unwrap();
    let contents: Vec<&[u8]> = contents.iter().map(|s| s.as_bytes()).collect();
    let contents = pad(&contents, 1, b' ');

    let mut regions: Vec<Region> = Vec::new();
    let mut region_grid: Vec<Vec<Option<usize>>> =
        vec![vec![None; contents[0].len()]; contents.len()];

    for (y, row) in contents.iter().enumerate() {
        for (x, &plot) in row.iter().enumerate() {
            if plot == b' ' {
                continue;
            }

            if region_grid[y][x].is_none() {
                let mut visited: Vec<Vec<bool>> =
                    vec![vec![false; region_grid[0].len()]; region_grid.len()];
                let mut side_counted =
                    vec![vec![SidesCounted::new(); region_grid[0].len()]; region_grid.len()];

                regions.push(Region::new(plot));

                let rlast = regions.len() - 1;
                flood_fill(
                    &mut regions[rlast],
                    rlast,
                    &mut region_grid,
                    &contents,
                    (x, y),
                    &mut visited,
                    &mut side_counted,
                );
            }
        }
    }

    part1(&mut writer, &regions);
    part2(&mut writer, &regions);
}

fn part1<W: Write>(writer: &mut BufWriter<W>, regions: &Vec<Region>) {
    let mut total = 0;
    for r in regions {
        println!(
            "region of {} - area: {}, perimiter: {}",
            u8_to_string(r.plant),
            r.area,
            r.perimiter
        );
        total += r.area * r.perimiter;
    }

    printwriteln!(writer, "part 1: {}", total).unwrap();
}

fn part2<W: Write>(writer: &mut BufWriter<W>, regions: &Vec<Region>) {
    let mut total = 0;
    for r in regions {
        println!(
            "region of {} - area: {}, sides: {}",
            u8_to_string(r.plant),
            r.area,
            r.sides
        );
        total += r.area * r.sides;
    }

    printwriteln!(writer, "part 2: {}", total).unwrap();
}

fn flood_fill(
    region: &mut Region,
    region_index: usize,
    region_grid: &mut Vec<Vec<Option<usize>>>,
    contents: &Vec<Vec<u8>>,
    loc: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    side_counted: &mut Vec<Vec<SidesCounted>>,
) {
    let (x, y) = loc;
    if visited[y][x] {
        return;
    }

    if contents[y][x] != region.plant {
        return;
    }

    region_grid[y][x] = Some(region_index);

    region.area += 1;
    region.perimiter += get_perimiter(region.plant, contents, x, y);

    visited[y][x] = true;

    for dir in [
        (side_counted[y][x].up, Dir4::Up),
        (side_counted[y][x].left, Dir4::Left),
        (side_counted[y][x].down, Dir4::Down),
        (side_counted[y][x].right, Dir4::Right),
    ] {
        if !dir.0 {
            count_side(region, contents, loc, side_counted, dir.1);
        }
    }

    for loc in Dir4::iter_valid_usizes_deltas((x, y), (contents[0].len(), contents.len())) {
        flood_fill(
            region,
            region_index,
            region_grid,
            contents,
            loc,
            visited,
            side_counted,
        );
    }
}

fn get_perimiter(plant: u8, contents: &[Vec<u8>], x: usize, y: usize) -> u64 {
    let mut total = 0;
    for (x, y) in Dir4::iter_valid_usizes_deltas((x, y), (contents[0].len(), contents.len())) {
        if contents[y][x] != plant {
            total += 1;
        }
    }

    total
}

fn count_side(
    region: &mut Region,
    contents: &[Vec<u8>],
    loc: (usize, usize),
    side_counted: &mut [Vec<SidesCounted>],
    dir: Dir4,
) {
    let (x, y) = loc;
    let (move_dir_1, move_dir_2, next_x, next_y) = match dir {
        Dir4::Up | Dir4::Down => (Dir4::Left, Dir4::Right, x + 1, y),
        Dir4::Left | Dir4::Right => (Dir4::Up, Dir4::Down, x, y + 1),
    };

    let (wall_x, wall_y) = match dir {
        Dir4::Up => (x, y - 1),
        Dir4::Down => (x, y + 1),
        Dir4::Left => (x - 1, y),
        Dir4::Right => (x + 1, y),
    };

    if contents[wall_y][wall_x] != region.plant {
        println!(
            "region {} sides += 1 ({:?})",
            u8_to_string(region.plant),
            dir
        );

        region.sides += 1;

        count_side_internal(region, contents, loc, side_counted, dir, move_dir_1);
        count_side_internal(
            region,
            contents,
            (next_x, next_y),
            side_counted,
            dir,
            move_dir_2,
        );
    }
}

fn count_side_internal(
    region: &mut Region,
    contents: &[Vec<u8>],
    loc: (usize, usize),
    side_counted: &mut [Vec<SidesCounted>],
    wall_dir: Dir4,
    move_dir: Dir4,
) {
    let (mut x, mut y) = loc;
    println!(
        "original: {} {} (region {})",
        x,
        y,
        u8_to_string(region.plant),
    );

    loop {
        if contents[y][x] != region.plant {
            break;
        }

        match wall_dir {
            Dir4::Up => side_counted[y][x].up = true,
            Dir4::Down => side_counted[y][x].down = true,
            Dir4::Left => side_counted[y][x].left = true,
            Dir4::Right => side_counted[y][x].right = true,
        };
        println!("marked wall at {} {} (dir {:?}) counted", x, y, wall_dir);

        let (wall_x, wall_y) = wall_dir.apply_delta_to_usizes((x, y));
        if contents[wall_y][wall_x] == region.plant {
            break;
        }

        (x, y) = move_dir.apply_delta_to_usizes((x, y));
    }
}
