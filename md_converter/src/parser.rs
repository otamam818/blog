use std::collections::HashSet;
use crate::data_models::{MarkdownData, MarkdownForm, INDENT_SPACES};

pub fn get_md_vec(data: &str) -> Vec<MarkdownData> {
    use MarkdownForm::*;
    let non_plains = HashSet::from(['#', '-', '`']);
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
                    PlainText => {
                        let prev = fin_vec.pop().unwrap();
                        fin_vec.push(MarkdownData {
                            form: PlainText,
                            inner_data: format!(
                                "{}\n{}",
                                prev.inner_data,
                                line
                            )
                        });
                    },
                    _ => fin_vec.push(parse_line(line))
                }
            },
            None => fin_vec.push(parse_line(line))
        };
    }

    fin_vec
}

pub fn parse_line(line: &str) -> MarkdownData {
    // Default values
    let line = line.to_string();
    let mut form = MarkdownForm::PlainText;
    let mut inner_data = line.clone();

    // Values to be re-assigned if it matches any other condition
    if line.starts_with("#") {
        let line_without_hash = line.trim_matches('#');
        let heading_number = line.len() - line_without_hash.len();
        assert!(heading_number <= 6);

        form = MarkdownForm::Heading(heading_number);
        inner_data = line_without_hash.trim().to_string();
    } else if line.trim().starts_with("-") {
        let indents = ( line.len() - line.trim().len() ) / INDENT_SPACES;

        form = MarkdownForm::UnorderedList(indents);
        inner_data = line
            .trim().trim_matches('-').trim()
            .to_string();
    }

    let inner_data = inner_data.to_owned();
    MarkdownData { form, inner_data }
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
}
