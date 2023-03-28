use {
    crate::label::Label,
    std::{collections::HashMap, fmt::Write},
};

// This function checks that references actually point to tags. If a missing tag is encountered, an
// error message is returned.
pub fn check(tags: &HashMap<String, Label>, refs: &[Label]) -> Result<(), String> {
    let mut error = String::new();
    let mut missing_tags = false;

    for r#ref in refs {
        if !tags.contains_key(&r#ref.label) {
            missing_tags = true;
            let _ = writeln!(error, "No tag found for {ref}.");
        }
    }

    if missing_tags {
        Err(error.trim().to_owned())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::{
            label::{Label, Type},
            references::check,
        },
        std::{collections::HashMap, path::Path},
    };

    #[test]
    fn check_empty() {
        let tags = HashMap::new();
        let refs = vec![];

        if let Err(error) = check(&tags, &refs) {
            panic!("{}", error);
        };
    }

    #[test]
    fn check_ok() {
        let mut tags = HashMap::new();
        tags.insert(
            "label1".to_owned(),
            Label {
                label_type: Type::Tag,
                label: "label1".to_owned(),
                description: String::new(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
        );

        let refs = vec![Label {
            label_type: Type::Ref,
            label: "label1".to_owned(),
            description: String::new(),
            path: Path::new("file1.rs").to_owned(),
            line_number: 1,
        }];

        if let Err(error) = check(&tags, &refs) {
            panic!("{}", error);
        };
    }

    #[test]
    fn check_missing() {
        let mut tags = HashMap::new();
        tags.insert(
            "label1".to_owned(),
            Label {
                label_type: Type::Tag,
                label: "label1".to_owned(),
                description: String::new(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
        );

        let refs = vec![
            Label {
                label_type: Type::Ref,
                label: "label1".to_owned(),
                description: String::new(),
                path: Path::new("file1.rs").to_owned(),
                line_number: 1,
            },
            Label {
                label_type: Type::Ref,
                label: "label2".to_owned(),
                description: String::new(),
                path: Path::new("file2.rs").to_owned(),
                line_number: 2,
            },
        ];

        match check(&tags, &refs) {
            Ok(()) => {
                panic!("The check(...) call should have failed.");
            }
            Err(error) => {
                assert!(error.contains(&refs[1].label));
            }
        };
    }
}
