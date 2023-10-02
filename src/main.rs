use nalgebra;
use std::cmp::max;

fn max_substring_len(a: &str, b: &str) -> usize {
    let mut lcs_matrix = nalgebra::DMatrix::<usize>::zeros(a.len(), b.len());

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a.char_indices().nth(i).unwrap().1 == b.char_indices().nth(j).unwrap().1 {
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

    dbg!(lcs_matrix).max()
}

fn main() {
    let a = "some string";
    let b = "another string";

    println!("{}", max_substring_len(a, b));
}
