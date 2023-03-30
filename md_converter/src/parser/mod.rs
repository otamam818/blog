mod parsers;
mod checkers;
mod flattener;

use crate::{
    data_models::{MarkdownData, MarkdownForm},
    parser::{checkers::*, parsers::*}
};

use self::flattener::flatten_md_data;

pub fn get_md_vec(data: &str) -> Vec<MarkdownData> {
    let fin_vec = data.lines()
        .map(|line| parse_line(line))
        .collect::<Vec<MarkdownData>>();

    // Flatten all adjacent MarkdownForm Lists
    let fin_vec = flatten_md_data(fin_vec);

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
    } else if let Some(tup) = parse_list(&line, &numbers) {
        // The checking and parsing of list tags (ordered or not) use some of
        // the same computations, so this pattern is used to prevent the need
        // to repeat these
        (form, inner_data) = tup;
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
            form: MarkdownForm::List {
                indents: 0,
                is_ordered: false,
                inner_bullet: None
            },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("  - Hello world");
        let data = MarkdownData {
            form: MarkdownForm::List {
                indents: 1,
                inner_bullet: None,
                is_ordered: false
            },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }

    #[test]
    fn ordered_list_understood() {
        let line = String::from("1. Hello world");
        let data = MarkdownData {
            form: MarkdownForm::List {
                indents: 0,
                inner_bullet: None,
                is_ordered: true
            },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);

        let line = String::from("  2. Hello world");
        let data = MarkdownData {
            form: MarkdownForm::List {
                indents: 1,
                is_ordered: true,
                inner_bullet: None
            },
            inner_data: "Hello world".to_string()
        };
        assert_eq!(parse_line(&line), data);
    }
}
