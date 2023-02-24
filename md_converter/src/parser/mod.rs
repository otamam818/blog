mod parsers;
mod checkers;

use std::collections::HashSet;
use crate::{
    data_models::{MarkdownData, MarkdownForm},
    parser::{checkers::*, parsers::*}
};

pub fn get_md_vec(data: &str) -> Vec<MarkdownData> {
    use MarkdownForm::*;
    let non_plains = HashSet::from([
        // Markdown's basic syntax, as outlined in John Gruber’s original
        // design document. All Markdown applications support these elements
        '#', '-', '`', '*', '>', '[', '!',
        // Support for numbers
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'
    ]);
    let mut fin_vec = Vec::new();
    for line in data.lines() {
        let first_char = line.trim_start().chars().nth(0).unwrap();
        if non_plains.contains(&first_char) {
            fin_vec.push(parse_line(line));
            continue;
        }

        match fin_vec.last() {
            Some(md_atom) => {
                match md_atom.form {
                    PlainText => modify_plaintext(&mut fin_vec, line),
                    _ => fin_vec.push(parse_line(line))
                }
            },
            None => fin_vec.push(parse_line(line))
        };
    }

    fin_vec
}

fn parse_line(line: &str) -> MarkdownData {
    let numbers
        = HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);
    // Default values
    let line = line.to_string();
    let mut form = MarkdownForm::PlainText;
    let mut inner_data = line.clone();

    // Values to be re-assigned if it matches any other condition
    if is_title(&line) {
        (form, inner_data) = parse_title(&line);
    } else if is_unordered_list(&line) {
        (form, inner_data) = parse_unordered_list(&line);
    } else if is_ordered_list(&line, &numbers) {
        if let Some(tup) = parse_ordered_list(&line, &numbers) {
            (form, inner_data) = tup;
        }
    }

    MarkdownData { form, inner_data }
}

fn modify_plaintext (fin_vec: &mut Vec<MarkdownData>, line: &str) {
    let prev = fin_vec.pop().unwrap();
    fin_vec.push(MarkdownData {
        form: MarkdownForm::PlainText,
        inner_data: format!(
            "{}\n{}",
            prev.inner_data,
            line
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_understood() {
        let line = String::from("# Hello world");
        let data = MarkdownData {
            form: MarkdownForm::Heading(1),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("## Secondary title");
        let data = MarkdownData {
            form: MarkdownForm::Heading(2),
            inner_data: "Secondary title".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }

    #[test]
    fn unordered_list_understood() {
        let line = String::from("- Hello world");
        let data = MarkdownData {
            form: MarkdownForm::UnorderedList(0),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("  - Hello world");
        let data = MarkdownData {
            form: MarkdownForm::UnorderedList(1),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }

    #[test]
    fn ordered_list_understood() {
        let line = String::from("1. Hello world");
        let data = MarkdownData {
            form: MarkdownForm::OrderedList((0, 1)),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("  2. Hello world");
        let data = MarkdownData {
            form: MarkdownForm::OrderedList((1, 2)),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }
}
