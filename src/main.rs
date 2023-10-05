use std::{env, process};

mod lcs;
mod unix_like;

mod cli {

    use std::fs::File;
    use std::io::{self, BufRead, Error, Read};
    use std::process;

    pub fn files_from_args_as_string(args: Vec<String>) -> Result<(String, String), Error> {
        if args.len() != 3 {
            eprintln!("Usage: {} <file_a> <file_b>", args[0]);
            process::exit(1);
        }

        let mut file_a = File::open(&args[1]).expect("Failed to open file A");
        let mut file_b = File::open(&args[2]).expect("Failed to open file B");

        let mut a = String::new();
        let mut b = String::new();

        file_a
            .read_to_string(&mut a)
            .expect("Failed to read file A");
        file_b
            .read_to_string(&mut b)
            .expect("Failed to read file B");

        Ok((a, b))
    }

    pub fn files_lines_from_args(args: Vec<String>) -> (Vec<String>, Vec<String>) {
        if args.len() != 3 {
            eprintln!("Usage: {} <file1> <file2>", args[0]);
            process::exit(1);
        }

        let file1_path = &args[1];
        let file2_path = &args[2];

        let file1_lines = read_lines(file1_path);
        let file2_lines = read_lines(file2_path);

        match (file1_lines, file2_lines) {
            (Ok(lines1), Ok(lines2)) => (lines1, lines2),
            (Err(e), _) | (_, Err(e)) => {
                eprintln!("Error reading files: {}", e);
                process::exit(1);
            }
        }
    }

    fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
        let file = File::open(file_path)?;
        let lines: Vec<String> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

        Ok(lines)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <mode: lcs/unix_like> <file1> <file2>", args[0]);
        process::exit(1);
    }

    let mode = &args[1];
    let file1 = &args[2];
    let file2 = &args[3];

    match mode.as_str() {
        "lcs" => {
            let (a, b) = cli::files_from_args_as_string(vec![
                args[0].clone(),
                file1.to_string(),
                file2.to_string(),
            ])
            .unwrap();
            let diff = lcs::text_diff(&a, &b);
            println!("{}", diff);
        }
        "unix_like" => {
            let (a, b) = cli::files_lines_from_args(vec![
                args[0].clone(),
                file1.to_string(),
                file2.to_string(),
            ]);
            let diff = unix_like::text_diff(a, b);
            println!("{}", diff);
        }
        _ => {
            eprintln!("Invalid mode. Use 'lcs' or 'unix_like'.");
            process::exit(1);
        }
    }
}
