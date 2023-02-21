use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum MarkdownForm {
    Heading,
    UnorderedList,
    OrderedList,
    Code,
    PrewrittenHTML,
    PlainText
}

pub struct MarkdownData {
    pub form: MarkdownForm,
    pub inner_data: String
}

#[wasm_bindgen]
pub fn get_form(line: &str) -> MarkdownForm {
    if line.starts_with("#") {
        return MarkdownForm::Heading;
    }

    MarkdownForm::PlainText
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headings_understood() {
        let line = String::from("# Hello world");
        assert_eq!(get_form(&line), MarkdownForm::Heading);
    }
}
