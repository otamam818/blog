use std::collections::HashSet;
use crate::data_models::{MarkdownForm, INDENT_SPACES};

pub fn parse_title(line: &str) -> (MarkdownForm, String) {
    let line_without_hash = line.trim_matches('#');
    let heading_number = line.len() - line_without_hash.len();
    assert!(heading_number <= 6);

    (
        MarkdownForm::Heading{ heading_number },
        line_without_hash.trim().to_string()
    )
}

pub fn parse_list(line: &str, numbers: &HashSet<char>) -> Option<(MarkdownForm, String)> {
    let mut candidate_line = line.trim();

    let is_unordered = candidate_line.starts_with("- ");
    if is_unordered {
        let inner_data = candidate_line[2..candidate_line.len()].to_string();
        let indents = ( line.len() - line.trim().len() ) / INDENT_SPACES;
        let form = MarkdownForm::List {
            indents,
            is_ordered: false,
            inner_bullet: None
        };
        return Some((form, inner_data));
    }

    // Remove and collect trailing numbers 
    while numbers.contains(&get_nth_char(candidate_line, 1)) {
        candidate_line = &candidate_line[1..candidate_line.len()];
    }

    // Containing a `.` character after the set of numbers means its an
    // ordered list
    if let Some(curr_char) = candidate_line.chars().nth(0) {
        if curr_char == '.' {
            let indents = ( line.len() - line.trim().len() ) / INDENT_SPACES;
            let form = MarkdownForm::List {
                is_ordered: true,
                indents,
                inner_bullet: None
            };
            let inner_data
              = candidate_line[1..candidate_line.len()]
                .trim()
                .to_string();
            return Some((form, inner_data));
        } 
    }
    None
}

fn get_nth_char(line: &str, n: usize) -> char {
    line.chars().nth(n-1).unwrap()
}

