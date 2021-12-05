#![feature(path_try_exists)]

mod sol01;

fn main() {
    let accounts = ["gh", "sk"];
    let solvers = [
        (1, sol01::solve),
    ];

    print!("  ");
    for a in &accounts {
        print!("{:>5}", a);
    }
    println!();
    for (task, solve) in solvers {
        print!("{:02}", task);
        for acc in &accounts {
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