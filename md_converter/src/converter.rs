use wasm_bindgen::prelude::*;
use handlebars::Handlebars;
use serde_json::json;

use crate::{
    parser::get_md_vec,
    data_models::{MarkdownData, MarkdownForm}
};

#[wasm_bindgen]
pub fn convert_text(data: String) -> String {
    let md_vec = get_md_vec(&data);
    md_vec
        .iter()
        .map(|md_data| parse_md_data(md_data))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Parses the MarkdownData into a handlebars string
pub fn parse_md_data(md_data: &MarkdownData) -> String {
    let reg = Handlebars::new();
    match md_data.form {
        MarkdownForm::Heading { heading_number } => {
            reg.render_template(
                "<h{{num_heading}}> {{text}} </h{{num_heading}}>",
                &json!({
                    "num_heading" : heading_number,
                    "text" : md_data.inner_data
                })
            ).unwrap()
        },
        // TODO: Exhaust the match syntax until you no longer have to use `_`
        _ => panic!("Unimplemented")
    }
}

#[allow(dead_code)]
fn parse_list(_md_data: &MarkdownData, _reg: &Handlebars) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn makes_proper_headings() {
        let _line = String::from("# Hello title");
        let _result = "<h1> Hello title </h1>";
        todo!();
        /*
        assert_eq!(convert_text(_line), _result);

        let _line = String::from("## Hello subtitle");
        let _result = "<h2> Hello subtitle </h2>";
        todo!();
        assert_eq!(convert_text(_line), _result);
        */
    }
}
