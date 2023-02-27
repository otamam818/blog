mod parsers;
mod checkers;
mod flattener;

use crate::{
    data_models::{MarkdownData, MarkdownForm},
    parser::{checkers::*, parsers::*}
};

pub fn get_md_vec(data: &str) -> Vec<MarkdownData> {
    let fin_vec = data.lines()
        .map(|line| parse_line(line))
        .collect::<Vec<MarkdownData>>();

    /*
    let md_chars = SyntaxChecker::md_chars();
    let mut fin_vec: Vec<MarkdownData> = Vec::new();

    for line in data.lines() {
        let first_char = line.trim_start().chars().nth(0).unwrap();
        if md_chars.contains(&first_char) {
            fin_vec.push(parse_line(line));
            continue;
        }

        match fin_vec.last() {
            Some(md_atom) => {
                match md_atom.form {
                    MarkdownForm::PlainText { .. }
                        => modify_plaintext(&mut fin_vec, line),

                    _ => fin_vec.push(parse_line(line))
                }
            },
            None => fin_vec.push(parse_line(line))
        };
    }
    */

    // TODO: Flatten all adjacent MarkdownForm Lists

    fin_vec
}

pub fn parse_line(line: &str) -> MarkdownData {
    let numbers = SyntaxChecker::numbers();

    // Default values
    let line = line.to_string();
    let mut form = MarkdownForm::new_plaintext();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_understood() {
        let line = String::from("# Hello world");
        let data = MarkdownData {
            form: MarkdownForm::Heading { heading_number: 1 },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("## Secondary title");
        let data = MarkdownData {
            form: MarkdownForm::Heading { heading_number: 2 },
            inner_data: "Secondary title".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }

    #[test]
    fn unordered_list_understood() {
        let line = String::from("- Hello world");
        let data = MarkdownData {
            form: MarkdownForm::UnorderedList { indents: 0, inner_bullet: None },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("  - Hello world");
        let data = MarkdownData {
            form: MarkdownForm::UnorderedList { indents: 1, inner_bullet: None },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }

    #[test]
    fn ordered_list_understood() {
        let line = String::from("1. Hello world");
        let data = MarkdownData {
            form: MarkdownForm::OrderedList {
                indents: 0,
                current_number: 1,
                inner_bullet: None
            },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("  2. Hello world");
        let data = MarkdownData {
            form: MarkdownForm::OrderedList {
                indents: 1,
                current_number: 2,
                inner_bullet: None
            },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }
}
