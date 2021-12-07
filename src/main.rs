#![feature(path_try_exists, iter_partition_in_place)]

mod sol01;
mod sol02;
mod sol03;
mod sol04;
mod sol05;
mod sol06;

const ACCOUNTS: &[&str] = &["gh", "sk"];
#[allow(clippy::type_complexity)]
const SOLVERS: &[(i32, fn(&str, &mut dyn FnMut(String)))] = &[
    (1, sol01::solve),
    (2, sol02::solve),
    (3, sol03::solve),
    (4, sol04::solve),
    (5, sol05::solve),
    (6, sol06::solve),
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

            println!("{:02} {:>5}", task, acc);
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
            dbg!(times);
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
