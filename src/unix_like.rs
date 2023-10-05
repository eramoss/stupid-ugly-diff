pub fn text_diff(mut lines1: Vec<String>, mut lines2: Vec<String>) -> String {
    let mut removed = String::from("\x1b[31m");
    let mut added = String::from("\x1b[32m");

    if lines1.len() > lines2.len() {
        lines2.resize(lines1.len(), "".to_string());
    } else {
        lines1.resize(lines2.len(), "".to_string());
    }

    for (line1, line2) in lines1.into_iter().zip(lines2.into_iter()) {
        match (line1, line2) {
            (line1, line2) if line1 != line2 => {
                added.push_str(&format!(">\t {}\n", line2));
                removed.push_str(&format!("<\t {}\n", line1));
            }
            (_, _) => continue,
        }
    }
    removed.push_str("\n\x1b[0m --- \n");
    let diff = removed + added.as_str();
    diff
}
