use std::{fs::File, io};

pub fn diff(
    lines1: io::Lines<io::BufReader<File>>,
    lines2: io::Lines<io::BufReader<File>>,
) -> String {
    let mut removed = String::from("\x1b[31m");
    let mut added = String::from("\x1b[32m");

    for (line1, line2) in lines1.zip(lines2) {
        match (line1, line2) {
            (Ok(line1), Ok(line2)) if line1 != line2 => {
                added.push_str(&format!("> {}\n", line2));
                removed.push_str(&format!("< {}\n", line1));
            }
            (Ok(line1), Err(_)) => {
                removed.push_str(&format!("> {}\n", line1));
            }
            (Err(_), Ok(line2)) => {
                added.push_str(&format!("> {}\n", line2));
            }
            (Err(_), Err(_)) => break,
            (Ok(_), Ok(_)) => continue,
        }
    }
    removed.push_str("\n\x1b[0m --- \n");
    removed.push_str(added.as_str());
    removed
}
