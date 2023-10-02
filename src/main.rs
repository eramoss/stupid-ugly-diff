use nalgebra::{self, Dyn, Matrix, VecStorage};
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::Read;

type UsizeMatrix = Matrix<usize, Dyn, Dyn, VecStorage<usize, Dyn, Dyn>>;

fn construct_substring(a: &str, b: &str) -> String {
    let lcs_matrix: UsizeMatrix = construct_lcs_matrix(a, b);
    let mut substring = "".to_owned();
    let mut i: usize = a.len() - 1;
    let mut j: usize = b.len() - 1;
    loop {
        if j > 0 && i > 0 {
            if lcs_matrix[(i, j)] > lcs_matrix[(i, j - 1)]
                && lcs_matrix[(i, j)] > lcs_matrix[(i - 1, j)]
            {
                substring.push(a.chars().nth(i).unwrap());
                j = j - 1;
                i = i - 1;
            } else if lcs_matrix[(i - 1, j)] < lcs_matrix[(i, j - 1)] {
                j = j - 1;
            } else {
                i = i - 1;
            }
        } else {
            if lcs_matrix[(i, j)] == 1 {
                substring.push(a.chars().nth(i).unwrap());
            }
            break;
        }
    }
    substring.chars().rev().collect()
}

#[allow(dead_code)]
fn dbg_lcs_matrix(a: &str, b: &str, lcs_matrix: &UsizeMatrix) {
    println!("  {b}");
    for i in 0..lcs_matrix.shape().0 {
        print!("{} ", a.chars().nth(i).unwrap());
        for j in 0..lcs_matrix.shape().1 {
            print!("{}", lcs_matrix[(i, j)]);
        }
        print!("\n");
    }
}

fn construct_lcs_matrix(a: &str, b: &str) -> UsizeMatrix {
    use nalgebra::DMatrix;
    let mut lcs_matrix: UsizeMatrix = DMatrix::zeros(a.len(), b.len());

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a.chars().nth(i).unwrap() == b.chars().nth(j).unwrap() {
                if i == 0 || j == 0 {
                    lcs_matrix[(i, j)] = 1;
                    continue;
                }
                lcs_matrix[(i, j)] = 1 + lcs_matrix[(i - 1, j - 1)];
            } else {
                if i == 0 || j == 0 {
                    lcs_matrix[(i, j)] = 0;
                    continue;
                }
                lcs_matrix[(i, j)] = max(lcs_matrix[(i - 1, j)], lcs_matrix[(i, j - 1)]);
            }
        }
    }
    lcs_matrix
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <file_a> <file_b>", args[0]);
        std::process::exit(1);
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

    let common_substring = construct_substring(&a, &b);
    println!("{}", common_substring);
}
