use crate::data_models::{MarkdownForm, MarkdownData};

pub fn flatten_md_data(init_vec: Vec<MarkdownData>) -> Vec<MarkdownData> {
    let mut fin_vec = Vec::new();
    let mut vec_iter = init_vec.iter();
    let mut md_next = vec_iter.next();
    while let Some(md_atom) = md_next {
        flatten_plaintext(md_atom, &mut md_next, &mut vec_iter, &mut fin_vec);
        flatten_list(md_atom, &mut md_next, &mut vec_iter, &mut fin_vec);
    }

    fin_vec
}

fn flatten_list <'a> (
    mut md_atom: &'a MarkdownData,
    md_next: &mut Option<&'a MarkdownData>,
    vec_iter: &mut std::slice::Iter<'a, MarkdownData>,
    fin_vec: &mut Vec<MarkdownData>
) {
    // Collect all adjacent List objects into a separate vector
    let mut data_holder = Vec::new();
    while let MarkdownForm::List { .. } = md_atom.form {
        data_holder.push(md_atom);

        *md_next = vec_iter.next();
        match *md_next {
            Some(atom) => md_atom = atom,
            None => break
        };
    }

    // TODO: Make a new function that recursively makes the new struct
    match data_holder.pop() {
        None => (),
        Some(initial_data) => {
            fin_vec.push(flatten_list_vec(data_holder, initial_data))
        }
    }
    /*
    if data_holder.len() > 0 {
        let mut curr_md_form = data_holder.pop().expect("len > 0").clone();
        while data_holder.len() > 0 {
            let new_data = data_holder.pop().expect("len > 0");
            #[allow(unused_variables)]
            let form = if let MarkdownForm::List {
                indents,
                inner_bullet,
                is_ordered,
                next_bullet,
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
    */
}

fn destructure_list_values(container: &MarkdownData) -> (
    String, 
    usize, 
    bool, 
    &Option<Box<MarkdownData>>, 
    &Option<Box<MarkdownData>>
) {
    let (
        init_inner_data,
        init_indents,
        init_is_ordered,
        init_inner_bullet,
        init_next_bullet
    );
    let MarkdownData { form, inner_data } = container;
    if let MarkdownForm::List {
        indents,
        is_ordered,
        inner_bullet,
        next_bullet
    } = form {
        init_indents = indents;
        init_is_ordered = is_ordered;
        init_inner_bullet = inner_bullet;
        init_next_bullet = next_bullet;
    } else {
        panic!("If statement did not match");
    };
    init_inner_data = inner_data;
    (
        init_inner_data.clone(),
        init_indents.clone(),
        init_is_ordered.clone(),
        init_inner_bullet,
        init_next_bullet
    )
}

fn flatten_list_vec(data_holder: Vec<&MarkdownData>, initial_data: &MarkdownData) -> MarkdownData {
    let (next_bullet, inner_bullet);
    let (
        init_inner_data,
        init_indents,
        init_is_ordered,
        init_inner_bullet,
        init_next_bullet
    ) = destructure_list_values(initial_data);
    match data_holder.clone().pop() {
        None => initial_data.clone(),
        Some(list_atom) => {
            let (
                _fin_inner_data,
                fin_indents,
                _fin_is_ordered,
                _fin_inner_bullet,
                _fin_next_bullet
            ) = destructure_list_values(list_atom);
            let form: MarkdownForm;
            if init_indents == fin_indents {
                next_bullet = Some(Box::new(list_atom.clone()));
                form = MarkdownForm::List {
                    indents: init_indents,
                    is_ordered: init_is_ordered,
                    inner_bullet: init_inner_bullet.clone(),
                    next_bullet
                };
            } else {
                inner_bullet = Some(Box::new(list_atom.clone()));
                form = MarkdownForm::List {
                    indents: init_indents,
                    is_ordered: init_is_ordered,
                    inner_bullet,
                    next_bullet: init_next_bullet.clone(),
                };
            }

            MarkdownData {
                form,
                inner_data: init_inner_data
            }
        }
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
