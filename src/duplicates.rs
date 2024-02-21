use {
    crate::label::Label,
    std::{collections::HashMap, fmt::Write},
};

// This function checks that all the vectors in `tags_map` have at most one element. It returns a
// vector of error strings.
pub fn check(tags_map: &HashMap<String, Vec<Label>>) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for (tag, labels) in tags_map {
        if labels.len() > 1 {
            let mut error = String::new();
            let _ = writeln!(error, "Duplicate tags found for label `{tag}`:");
            for label in labels {
                let _ = writeln!(error, "  {label}");
            }
            errors.push(error);
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            duplicates::check,
            label::{Label, Type},
        },
        std::{collections::HashMap, path::Path},
    };

    #[test]
    fn check_empty() {
        assert!(check(&HashMap::new()).is_empty());
    }

    #[test]
    fn check_no_dupes() {
        let mut tags_map = HashMap::new();

        let tags_vec1 = vec![Label {
            label_type: Type::Tag,
            label: "label1".to_owned(),
            path: Path::new("file1.rs").to_owned(),
            line_number: 1,
        }];

        let tags_vec2 = vec![Label {
            label_type: Type::Tag,
            label: "label2".to_owned(),
            path: Path::new("file2.rs").to_owned(),
            line_number: 2,
        }];

        tags_map.insert("label1".to_owned(), tags_vec1);
        tags_map.insert("label2".to_owned(), tags_vec2);

        assert!(check(&tags_map).is_empty());
    }

    #[test]
    fn check_dupes() {
        let mut tags_map = HashMap::new();

        let tags_vec1 = vec![Label {
            label_type: Type::Tag,
            label: "label1".to_owned(),
            path: Path::new("file1.rs").to_owned(),
            line_number: 1,
        }];

        let tags_vec2 = vec![
            Label {
                label_type: Type::Tag,
                label: "label2".to_owned(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
            Label {
                label_type: Type::Tag,
                label: "label2".to_owned(),
                path: Path::new("file2.rs").to_owned(),
                line_number: 2,
            },
        ];

        let tags_vec3 = vec![
            Label {
                label_type: Type::Tag,
                label: "label3".to_owned(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
            Label {
                label_type: Type::Tag,
                label: "label3".to_owned(),
                path: Path::new("file2.rs").to_owned(),
                line_number: 2,
            },
            Label {
                label_type: Type::Tag,
                label: "label3".to_owned(),
                path: Path::new("file3.rs").to_owned(),
                line_number: 2,
            },
        ];

        tags_map.insert("label1".to_owned(), tags_vec1.clone());
        tags_map.insert("label2".to_owned(), tags_vec2.clone());
        tags_map.insert("label3".to_owned(), tags_vec3.clone());

        let errors = check(&tags_map);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains(&format!("{}", tags_vec2[0])));
        assert!(errors[0].contains(&format!("{}", tags_vec2[1])));
        assert!(errors[1].contains(&format!("{}", tags_vec3[0])));
        assert!(errors[1].contains(&format!("{}", tags_vec3[1])));
        assert!(errors[1].contains(&format!("{}", tags_vec3[2])));
    }
}
