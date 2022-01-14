#![feature(path_try_exists)]
#![feature(iter_partition_in_place)]
#![feature(let_else)]
#![feature(destructuring_assignment)]

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
];

fn run() {
    print!("  ");
    for a in ACCOUNTS {
        print!("{:>5}", a);
    }
    println!();
    for &(task, solve) in SOLVERS {
        print!("{:02}", task);
        for acc in ACCOUNTS {
            let input_path = format!("data/{}/{:02}.in", acc, task);
            let output_path = format!("data/{}/{:02}.out", acc, task);
            if std::fs::try_exists(&input_path).unwrap() {
                let input = std::fs::read_to_string(input_path).unwrap();
                let mut output = String::new();
                let mut out = |s: String| { output.push_str(&s); output.push('\n'); };
                solve(&input, &mut out);
                if std::fs::try_exists(&output_path).unwrap() {
                    let expected_output = std::fs::read_to_string(output_path).unwrap();
                    assert_eq!(output, expected_output);
                } else {
                    dbg!(output);
                }
                print!("{:>5}", "ok");
            } else {
                print!("{:>5}", "--");
            }
        }
        println!();
    }
}

fn bench() {
    for &(task, solve) in SOLVERS {
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

            assert_eq!(output, expected_output);
            println!("    {:?}", times);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match args.as_slice() {
        [] => run(),
        ["bench"] => bench(),
        _ => panic!(),
    }
}
