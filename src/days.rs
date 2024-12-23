mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

pub fn run(day: u8) {
    match day {
        1 => day_01::run(),
        2 => day_02::run(),
        3 => day_03::run(),
        4 => day_04::run(),
        5 => day_05::run(),
        6 => day_06::run(),
        7 => day_07::run(),
        8 => day_08::run(),
        9 => day_09::run(),
        10 => day_10::run(),
        11 => day_11::run(),
        12 => day_12::run(),
        13 => day_13::run(),
        14 => day_14::run(),
        15 => day_15::run(),
        16 => day_16::run(),
        17 => day_17::run(),
        18 => day_18::run(),
        19 => day_19::run(),
        20 => day_20::run(),
        21 => day_21::run(),
        22 => day_22::run(),
        23 => day_23::run(),
        24 => day_24::run(),
        25 => day_25::run(),
        0 | 26..=u8::MAX => panic!("invalid day"),
    };
}
