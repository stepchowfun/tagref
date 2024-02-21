use {crate::label::Label, std::collections::HashSet};

// This function checks that tag references actually point to tags. It returns a vector of error
// strings.
pub fn check(tags: &HashSet<String>, refs: &[Label]) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for r#ref in refs {
        if !tags.contains(&r#ref.label) {
            errors.push(format!("No tag found for {ref}."));
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            label::{Label, Type},
            tag_references::check,
        },
        std::{collections::HashSet, path::Path},
    };

    #[test]
    fn check_empty() {
        let tags = HashSet::<String>::new();
        let refs = vec![];

        assert!(check(&tags, &refs).is_empty());
    }

    #[test]
    fn check_ok() {
        let mut tags = HashSet::new();
        tags.insert("label1".to_owned());

        let refs = vec![Label {
            label_type: Type::Ref,
            label: "label1".to_owned(),
            path: Path::new("file1.rs").to_owned(),
            line_number: 1,
        }];

        assert!(check(&tags, &refs).is_empty());
    }

    #[test]
    fn check_missing() {
        let mut tags = HashSet::new();
        tags.insert("label1".to_owned());

        let refs = vec![
            Label {
                label_type: Type::Ref,
                label: "label1".to_owned(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
            Label {
                label_type: Type::Ref,
                label: "label2".to_owned(),
                path: Path::new("file2.rs").to_owned(),
                line_number: 2,
            },
            Label {
                label_type: Type::Ref,
                label: "label3".to_owned(),
                path: Path::new("file3.rs").to_owned(),
                line_number: 3,
            },
        ];

        let errors = check(&tags, &refs);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains(&refs[1].label));
        assert!(errors[1].contains(&refs[2].label));
    }
}
