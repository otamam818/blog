use wasm_bindgen::prelude::*;

const INDENT_SPACES: usize = 2;

type Indentations = usize;

#[derive(PartialEq, Debug)]
pub enum MarkdownForm {
    Heading,
    /// UnorderedLists and OrderedLists contains a set number of spaces to
    /// identify its indentation
    UnorderedList(Indentations),
    OrderedList(Indentations),
    Code,
    PrewrittenHTML,
    PlainText
}

#[derive(PartialEq, Debug)]
pub struct MarkdownData {
    pub form: MarkdownForm,
    pub inner_data: String
}

#[wasm_bindgen]
pub fn convert_text(data: String) -> String {
    // Make it a Vec<MarkdownData> data
    // Parse the Vec<MarkdownData> data into a handlebars file
    format!("TODO: Parse the following\n{}", data)
}

pub fn get_form(line: &str) -> MarkdownData {
    // Default values
    let line = line.to_string();
    let mut form = MarkdownForm::PlainText;
    let mut inner_data = line.clone();

    // Values to be re-assigned if it matches any other condition
    if line.starts_with("#") {
        form = MarkdownForm::Heading;
        inner_data = line.trim_matches('#').trim().to_string();
    } else if line.trim().starts_with("-") {
        let indents = ( line.len() - line.trim().len() ) / INDENT_SPACES;
        form = MarkdownForm::UnorderedList(indents);

        inner_data = line.trim().trim_matches('-').trim().to_string();
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
            form: MarkdownForm::Heading,
            inner_data: "Hello world".to_string()
        };
        assert_eq!(get_form(&line), data);
    }

    #[test]
    fn unordered_list_understood() {
        let line = String::from("- Hello world");
        let data = MarkdownData {
            form: MarkdownForm::UnorderedList(0),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(get_form(&line), data);

        let line = String::from("  - Hello world");
        let data = MarkdownData {
            form: MarkdownForm::UnorderedList(1),
            inner_data: "Hello world".to_string()
        };
        assert_eq!(get_form(&line), data);
    }
}
