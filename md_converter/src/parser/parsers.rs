use std::collections::HashSet;
use crate::data_models::{MarkdownForm, INDENT_SPACES};

pub fn parse_unordered_list(line: &str) -> (MarkdownForm, String) {
    let indents = ( line.len() - line.trim().len() ) / INDENT_SPACES;

    (
        MarkdownForm::UnorderedList(indents),
        line.trim().trim_matches('-').trim().to_string()
    )
}

pub fn parse_title(line: &str) -> (MarkdownForm, String) {
    let line_without_hash = line.trim_matches('#');
    let heading_number = line.len() - line_without_hash.len();
    assert!(heading_number <= 6);

    (
        MarkdownForm::Heading(heading_number),
        line_without_hash.trim().to_string()
    )
}

pub fn parse_ordered_list(line: &str, numbers: &HashSet<char>) -> Option<(MarkdownForm, String)> {
    let mut candidate_line = line.trim();
    let mut curr_num = Vec::new();

    // Remove and collect trailing numbers 
    while numbers.contains(&candidate_line.chars().nth(0).unwrap()) {
        curr_num.push(candidate_line.chars().nth(0).unwrap());
        candidate_line = &candidate_line[1..candidate_line.len()];
    }

    // Containing a `.` character after the set of numbers means its an
    // ordered list
    if let Some(curr_char) = candidate_line.chars().nth(0) {
        if curr_char == '.' {
            let curr_num: usize = curr_num.iter()
                .collect::<String>()
                .parse()
                .expect("\
                    Only numbers were supposed to have been collected
                    previously");
            let indents = ( line.len() - line.trim().len() )
                / INDENT_SPACES;

            let form = MarkdownForm::OrderedList((indents, curr_num));
            let inner_data
              = candidate_line[1..candidate_line.len()]
                .trim()
                .to_string();
            return Some((form, inner_data));
        } 
    }
    None
}

