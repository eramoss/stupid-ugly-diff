use nalgebra::{self, Dyn, Matrix, VecStorage};
use std::cmp::max;
type UsizeMatrix = Matrix<usize, Dyn, Dyn, VecStorage<usize, Dyn, Dyn>>;

pub fn text_diff(a: &str, b: &str) -> String {
    let lcs_result = lcs(a, b);
    let mut diff = String::new();
    let mut a_idx = 0;
    let mut b_idx = 0;

    for change in lcs_result.chars() {
        while a.chars().nth(a_idx) != Some(change) {
            diff.push_str(&format!("\x1b[32m{}\x1b[0m", a.chars().nth(a_idx).unwrap()));
            a_idx += 1;
        }

        while b.chars().nth(b_idx) != Some(change) {
            diff.push_str(&format!("\x1b[31m{}\x1b[0m", b.chars().nth(b_idx).unwrap()));
            b_idx += 1;
        }

        diff.push(change);
        a_idx += 1;
        b_idx += 1;
    }

    diff
}

pub fn lcs(a: &str, b: &str) -> String {
    let lcs_matrix: UsizeMatrix = lcs_matrix(a, b);
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

pub fn lcs_matrix(a: &str, b: &str) -> UsizeMatrix {
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
#[allow(dead_code)]
pub fn dbg_lcs_matrix(a: &str, b: &str, lcs_matrix: &UsizeMatrix) {
    println!("  {b}");
    for i in 0..lcs_matrix.shape().0 {
        print!("{} ", a.chars().nth(i).unwrap());
        for j in 0..lcs_matrix.shape().1 {
            print!("{}", lcs_matrix[(i, j)]);
        }
        print!("\n");
    }
}
