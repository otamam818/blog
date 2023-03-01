use crate::data_models::{MarkdownForm, MarkdownData};

// TODO: Use this code eventually in the `get_md_vec` function
#[allow(dead_code)]
pub fn flatten_md_data(init_vec: Vec<MarkdownData>) -> Vec<MarkdownData> {
    let mut fin_vec = Vec::new();
    let mut vec_iter = init_vec.iter();
    let mut md_next = vec_iter.next();
    while let Some(md_atom) = md_next {
        flatten_plaintext(md_atom, &mut md_next, &mut vec_iter, &mut fin_vec);
        flatten_unordered_list(md_atom, &mut md_next, &mut vec_iter, &mut fin_vec);
    }

    fin_vec
}

fn flatten_unordered_list <'a> (
    mut md_atom: &'a MarkdownData,
    md_next: &mut Option<&'a MarkdownData>,
    vec_iter: &mut std::slice::Iter<'a, MarkdownData>,
    fin_vec: &mut Vec<MarkdownData>
) {
    let mut data_holder = Vec::new();
    while let MarkdownForm::List { .. } = md_atom.form {
        data_holder.push(md_atom);

        *md_next = vec_iter.next();
        match *md_next {
            Some(atom) => md_atom = atom,
            None => break
        };
    }

    if data_holder.len() > 0 {
        let mut curr_md_form = data_holder.pop().expect("len > 0").clone();
        while data_holder.len() > 0 {
            let new_data = data_holder.pop().expect("len > 0");
            #[allow(unused_variables)]
            let form = if let MarkdownForm::List {
                indents,
                inner_bullet,
                is_ordered
            } = &new_data.form {
                // Inner bullet should be None at this point anyways
                let inner_bullet = Some(Box::new(curr_md_form));
                MarkdownForm::List {
                    indents: *indents,
                    inner_bullet,
                    is_ordered: *is_ordered
                }
            } else {
                panic!("First if statement didn't get true by default");
            };
            curr_md_form = MarkdownData {
                form,
                inner_data: new_data.inner_data.clone()
            }
        }
        fin_vec.push(curr_md_form);
    }
}

fn flatten_plaintext <'a> (
    mut md_atom: &'a MarkdownData,
    md_next: &mut Option<&'a MarkdownData>,
    vec_iter: &mut std::slice::Iter<'a, MarkdownData>,
    fin_vec: &mut Vec<MarkdownData>
) {
    let mut data_holder = Vec::new();
    while let MarkdownForm::PlainText { has_line_break } = md_atom.form {
        data_holder.push(
            MarkdownData {
                form: MarkdownForm::PlainText { has_line_break },
                inner_data: md_atom.inner_data.clone()
            }
        );

        *md_next = vec_iter.next();
        match *md_next {
            Some(atom) => md_atom = atom,
            None => break
        };
    }

    if data_holder.len() > 0 {
        let mut curr_md_form = data_holder.pop().expect("len > 0");
        while data_holder.len() > 0 {
            let new_data = data_holder.pop().expect("len > 0");
            curr_md_form = MarkdownData {
                form: curr_md_form.form,
                inner_data: format!(
                    "{} {}",
                    new_data.inner_data,
                    curr_md_form.inner_data
                )
            }
        }
        fin_vec.push(curr_md_form);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattens_plaintext() {
        let data = vec![
            MarkdownData {
                form: MarkdownForm::PlainText { has_line_break: true },
                inner_data: "Line 1".to_string()
            },
            MarkdownData {
                form: MarkdownForm::PlainText { has_line_break: true },
                inner_data: "Still line 1".to_string()
            }
        ];
        let data = flatten_md_data(data);
        let data = data.get(0).expect("Data made manually");
        assert_eq!(data.inner_data, "Line 1 Still line 1".to_string());
        assert_eq!(data.form, MarkdownForm::PlainText { has_line_break: true });
    }

    #[test]
    fn flattens_unordered_list() {
        let data = vec![
            MarkdownData {
                form: MarkdownForm::List {
                    indents: 0,
                    inner_bullet: None,
                    is_ordered: false
                },
                inner_data: "Outer Bullet".to_string()
            },
            MarkdownData {
                form: MarkdownForm::List {
                    indents: 1,
                    inner_bullet: None,
                    is_ordered: false
                },
                inner_data: "Inner Bullet".to_string()
            }
        ];
        let data = flatten_md_data(data);
        let data = data.get(0).expect("Data made manually").clone();
        assert_eq!(data, MarkdownData {
            form: MarkdownForm::List {
                indents: 0,
                is_ordered: false,
                inner_bullet: Some(Box::new(MarkdownData {
                    form: MarkdownForm::List {
                        indents: 1,
                        inner_bullet: None,
                        is_ordered: false
                    },
                    inner_data: "Inner Bullet".to_string()
                }))
            },
            inner_data: "Outer Bullet".to_string(),
        });
    }
}
