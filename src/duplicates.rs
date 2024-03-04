use {
    crate::directive::Directive,
    std::{collections::HashMap, fmt::Write},
};

// This function checks that all the vectors in `tags_map` have at most one element. It returns a
// vector of error strings.
pub fn check(tags_map: &HashMap<String, Vec<Directive>>) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for (label, directives) in tags_map {
        if directives.len() > 1 {
            let mut error = String::new();
            let _ = writeln!(error, "Duplicate tags found for label `{label}`:");
            for directive in directives {
                let _ = writeln!(error, "  {directive}");
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
            directive::{Directive, Type},
            duplicates::check,
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

        let tags_vec1 = vec![Directive {
            r#type: Type::Tag,
            label: "tag1".to_owned(),
            path: Path::new("file1.rs").to_owned(),
            line_number: 1,
        }];

        let tags_vec2 = vec![Directive {
            r#type: Type::Tag,
            label: "tag2".to_owned(),
            path: Path::new("file2.rs").to_owned(),
            line_number: 2,
        }];

        tags_map.insert("tag1".to_owned(), tags_vec1);
        tags_map.insert("tag2".to_owned(), tags_vec2);

        assert!(check(&tags_map).is_empty());
    }

    #[test]
    fn check_dupes() {
        let mut tags_map = HashMap::new();

        let tags_vec1 = vec![Directive {
            r#type: Type::Tag,
            label: "tag1".to_owned(),
            path: Path::new("file1.rs").to_owned(),
            line_number: 1,
        }];

        let tags_vec2 = vec![
            Directive {
                r#type: Type::Tag,
                label: "tag2".to_owned(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
            Directive {
                r#type: Type::Tag,
                label: "tag2".to_owned(),
                path: Path::new("file2.rs").to_owned(),
                line_number: 2,
            },
        ];

        let tags_vec3 = vec![
            Directive {
                r#type: Type::Tag,
                label: "tag3".to_owned(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
            Directive {
                r#type: Type::Tag,
                label: "tag3".to_owned(),
                path: Path::new("file2.rs").to_owned(),
                line_number: 2,
            },
            Directive {
                r#type: Type::Tag,
                label: "tag3".to_owned(),
                path: Path::new("file3.rs").to_owned(),
                line_number: 2,
            },
        ];

        tags_map.insert("tag1".to_owned(), tags_vec1.clone());
        tags_map.insert("tag2".to_owned(), tags_vec2.clone());
        tags_map.insert("tag3".to_owned(), tags_vec3.clone());

        let errors = check(&tags_map);
        assert_eq!(errors.len(), 2);
        assert!(
            (errors[0].contains(&format!("{}", tags_vec2[0]))
                && errors[0].contains(&format!("{}", tags_vec2[1]))
                && errors[1].contains(&format!("{}", tags_vec3[0]))
                && errors[1].contains(&format!("{}", tags_vec3[1]))
                && errors[1].contains(&format!("{}", tags_vec3[2])))
                || (errors[0].contains(&format!("{}", tags_vec3[0]))
                    && errors[0].contains(&format!("{}", tags_vec3[1]))
                    && errors[0].contains(&format!("{}", tags_vec3[2]))
                    && errors[1].contains(&format!("{}", tags_vec2[0]))
                    && errors[1].contains(&format!("{}", tags_vec2[1]))),
        );
    }
}
