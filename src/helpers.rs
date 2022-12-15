use std::sync::atomic::{AtomicBool, Ordering::*};
use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

static EXAMPLE: AtomicBool = AtomicBool::new(false);

pub fn is_example() -> bool {
    EXAMPLE.load(Relaxed)
}

pub fn set_example() {
    EXAMPLE.store(true, Relaxed);
}

/// This macro helps make defining and running the problems for each day simpler
macro_rules! advent {
    ($(day $day_num:literal)+) => {

        use paste::paste;
        use clap::Parser;

        $(
            paste! { mod [<day_ $day_num>]; }
            advent!(__internal $day_num 1);
            advent!(__internal $day_num 2);
        )+


        paste! {
            #[derive(Parser, Debug)]
            struct Args {
                #[arg(short = 'x', long, default_value_t = false)]
                example: bool,

                #[arg(long = "p1")]
                problem_1: bool,

                #[arg(long = "p2")]
                problem_2: bool,

                $(
                    #[arg(long = concat!("day-", $day_num))]
                        [<day_ $day_num>]: bool,

                )*
            }
        }

        fn run() {

            let args = Args::parse();

            paste! {
                let run_all_days = true $(
                    && !args.[<day_ $day_num>]
                )*;
            }

            if args.example {
                $crate::helpers::set_example();
            }

            let run_all_problems = (!args.problem_1) && (!args.problem_2);

            paste! { $(
                if run_all_days || paste! { args.[<day_ $day_num>] } {

                    if run_all_problems || args.problem_1 {
                        if args.example {
                            [<day_ $day_num _problem_1_example>]();
                        } else {
                            [<day_ $day_num _problem_1>]();
                        }
                    }

                    if run_all_problems || args.problem_2 {
                        if args.example {
                            [<day_ $day_num _problem_2_example>]();
                        } else {
                            [<day_ $day_num _problem_2>]();
                        }
                    }
                }
            )+ }
        }
    };


    (__internal $day_num:literal $problem_num:literal) => {
        paste! {

            fn [<day_ $day_num _problem_ $problem_num>]() {
                print!("🎄 Day {}\t Problem {}  ", $day_num, $problem_num);
                let file = concat!(
                    "input/day_",
                    $day_num,
                    ".txt");

                let lines = $crate::helpers::read_lines(file).ok()
                    .unwrap_or_else(|| {
                        $crate::helpers::read_lines("input/empty.txt").expect(
                            "Failed to open input/empty.txt"
                        )
                    });

                let start = std::time::Instant::now();
                let result = [<day_ $day_num>]::[<problem_ $problem_num>](
                    lines.map(|line| line.expect(
                        concat!(
                            "Failed to read line from input/day_",
                            $day_num,
                            ".txt"
                        ))
                    )
                );

                let dur = start.elapsed();
                println!("  🎊 🎉 -> {}\t{}µs", result, dur.as_micros());
            }


            fn [<day_ $day_num _problem_ $problem_num _example>]() {
                print!("🎄 Day {}\t Problem {}  ", $day_num, $problem_num);
                let file = concat!(
                    "input/day_",
                    $day_num,
                    "_example.txt");

                let lines = $crate::helpers::read_lines(file).ok()
                    .unwrap_or_else(|| {
                        $crate::helpers::read_lines("input/empty.txt").expect(
                            "Failed to open input/empty.txt"
                        )
                    });

                let start = std::time::Instant::now();
                let result = [<day_ $day_num>]::[<problem_ $problem_num>](
                    lines.map(|line| line.expect(
                        concat!(
                            "Failed to read line from input/day_",
                            $day_num,
                            "_example
                            .txt"
                        ))
                    )
                );

                let dur = start.elapsed();
                println!("  🎊 🎉 -> {}\t{}µs", result, dur.as_micros());
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
