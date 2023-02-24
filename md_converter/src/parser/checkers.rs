use std::collections::HashSet;

pub fn is_title(line: &str) -> bool {
    line.starts_with("#")
}

pub fn is_unordered_list(line: &str) -> bool {
    line.trim().starts_with("-")
}

pub fn is_ordered_list(line: &str, numbers: &HashSet<char>) -> bool {
    numbers.contains(&line.trim().chars().nth(0).unwrap())
}

