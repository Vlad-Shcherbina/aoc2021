#![feature(fs_try_exists)]
#![feature(iter_partition_in_place)]

mod logger;
mod sol01;
mod sol02;
mod sol03;
mod sol04;
mod sol05;
mod sol06;
mod sol07;
mod sol08;
mod sol09;
mod sol10;
mod sol11;
mod sol12;
mod sol13;
mod sol14;
mod sol15;
mod sol16;
mod sol17;
mod sol18;
mod sol19;
mod sol20;
mod sol21;
mod sol22;
mod sol23;
mod sol24;
mod sol25;

const ACCOUNTS: &[&str] = &["gh", "sk"];
#[allow(clippy::type_complexity)]
const SOLVERS: &[(i32, fn(&str, &mut dyn FnMut(String)))] = &[
    (1, sol01::solve),
    (2, sol02::solve),
    (3, sol03::solve),
    (4, sol04::solve),
    (5, sol05::solve),
    (6, sol06::solve),
    (7, sol07::solve),
    (8, sol08::solve),
    (9, sol09::solve),
    (10, sol10::solve),
    (11, sol11::solve),
    (12, sol12::solve),
    (13, sol13::solve),
    (14, sol14::solve),
    (15, sol15::solve),
    (16, sol16::solve),
    (17, sol17::solve),
    (18, sol18::solve),
    (19, sol19::solve),
    (20, sol20::solve),
    (21, sol21::solve),
    (22, sol22::solve),
    (23, sol23::solve),
    (24, sol24::solve),
    (25, sol25::solve),
];

fn run(task_to_run: i32, generate: bool) {
    log::set_boxed_logger(Box::new(logger::TimeDeltaLogger::default())).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    for &(task, solve) in SOLVERS {
        if task != task_to_run {
            continue;
        }
        for acc in ACCOUNTS {
            log::info!("Day {:02}, {}", task, acc);
            let input_path = format!("data/{}/{:02}.in", acc, task);
            let output_path = format!("data/{}/{:02}.out", acc, task);
            if !std::fs::try_exists(&input_path).unwrap() {
                continue;
            }

            let input = std::fs::read_to_string(input_path).unwrap();
            let mut output = String::new();
            let mut out = |s: String| {
                output.push_str(&s); output.push('\n');
                log::info!("OUT: {}", s);
            };

            log::info!("solving...");
            solve(&input, &mut out);
            log::info!("done");

            if generate {
                std::fs::write(output_path, output).unwrap();
            } else if std::fs::try_exists(&output_path).unwrap() {
                let expected_output = std::fs::read_to_string(output_path).unwrap();
                if output != expected_output {
                    log::info!("does not match expected output");
                    log::info!("{}", expected_output);
                }
            } else {
                log::info!("output file does not exist");
            }
            eprintln!();
        }
    }
}

fn bench(tasks: &[i32]) {
    for &(task, solve) in SOLVERS {
        if !tasks.contains(&task) {
            continue;
        }
        for acc in ACCOUNTS {
            let input_path = format!("data/{}/{:02}.in", acc, task);
            let output_path = format!("data/{}/{:02}.out", acc, task);
            if !std::fs::try_exists(&input_path).unwrap() ||
               !std::fs::try_exists(&output_path).unwrap() {
                continue;
            }

            print!("{:02} {:>5}", task, acc);
            let input = std::fs::read_to_string(input_path).unwrap();
            let expected_output = std::fs::read_to_string(output_path).unwrap();

            let mut output = String::new();
            let mut times = Vec::with_capacity(2);
            let start = std::time::Instant::now();
            let mut out = |s: String| {
                times.push(start.elapsed());
                output.push_str(&s); output.push('\n');
            };
            solve(&input, &mut out);
            times.push(start.elapsed());

            assert_eq!(output, expected_output);
            println!("    {:?}", times);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match args.as_slice() {
        ["bench"] => bench(&(1..=25).collect::<Vec<_>>()),
        ["bench", task] => bench(&[task.parse().unwrap()]),
        [task] => run(task.parse().unwrap(), false),
        [task, "gen"] => run(task.parse().unwrap(), true),
        _ => panic!()
    }
}
