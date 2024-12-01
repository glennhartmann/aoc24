macro_rules! printwriteln {
    ($writer:expr, $fmt:literal) => {
        {
            println!($fmt);
            writeln!($writer, $fmt)
        }
    };
    ($writer:expr, $fmt:literal, $($args:expr),+) => {
        {
            println!($fmt, $($args),+);
            writeln!($writer, $fmt, $($args),+)
        }
    };
}

pub(crate) use printwriteln;
