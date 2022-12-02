use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

/// This macro helps make defining and running the problems for each day simpler
macro_rules! advent {
    ($(day $day_num:literal)+) => {

        use paste::paste;
        $(
            paste! { mod [<day_ $day_num>]; }
            advent!(__internal $day_num 1);
            advent!(__internal $day_num 2);
        )+

        fn run_all() {
            $(
                paste! {
                    [<day_ $day_num _problem_1>]();
                    [<day_ $day_num _problem_2>]();
                }
            )+
        }
    };

    (__internal $day_num:literal $problem_num:literal) => {
        paste! {

            fn [<day_ $day_num _problem_ $problem_num>]() {
                print!("🎄 Day {}\t Problem {}  ", $day_num, $problem_num);
                let file = concat!(
                    "input/day_",
                    $day_num,
                    '_',
                    $problem_num,
                    ".txt");

                let lines = $crate::helpers::read_lines(file).ok()
                    .unwrap_or_else(|| {
                        $crate::helpers::read_lines("input/empty.txt").expect(
                            "Failed to open input/empty.txt"
                        )
                    });

                let result = [<day_ $day_num>]::[<problem_ $problem_num>](
                    lines.map(|line| line.expect(
                        concat!(
                            "Failed to read line from input/day_",
                            $day_num,
                            '_',
                            $problem_num,
                            ".txt"
                        ))
                    )
                );

                println!("  🎊 🎉 -> {}", result);
            }
        }
    };
}

pub(crate) use advent;

pub fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
