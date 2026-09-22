use crate::directive::Directive;
use std::{collections::HashMap, fmt::Write};

// This function checks that tags and groups do not share labels. It returns a vector of error
// strings.
pub fn check(
    tags: &HashMap<String, Vec<Directive>>,
    groups: &HashMap<String, Vec<Directive>>,
) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for (label, tags) in tags {
        if let Some(groups) = groups.get(label) {
            let mut error = String::new();
            let _ = writeln!(error, "Label `{label}` is used by both a tag and a group:");
            for directive in tags.iter().chain(groups) {
                let _ = writeln!(error, "  {directive}");
            }
            errors.push(error);
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use crate::{
        directive::{Directive, Type},
        target_collisions::check,
    };
    use std::{collections::HashMap, path::Path};

    // This helper constructs a directive for the validation tests.
    fn directive(r#type: Type, label: &str, line_number: usize) -> Directive {
        Directive {
            r#type,
            label: label.to_owned(),
            path: Path::new("file.rs").to_owned(),
            line_number,
        }
    }

    #[test]
    fn check_disjoint() {
        // Construct a tag and group with different labels.
        let tags = HashMap::from([("tag".to_owned(), vec![directive(Type::Tag, "tag", 1)])]);
        let groups =
            HashMap::from([("group".to_owned(), vec![directive(Type::Group, "group", 2)])]);

        // Ensure disjoint target labels are valid.
        assert!(check(&tags, &groups).is_empty());
    }

    #[test]
    fn check_collision() {
        // Construct a tag and group with the same label.
        let tag = directive(Type::Tag, "shared", 1);
        let group = directive(Type::Group, "shared", 2);
        let tags = HashMap::from([("shared".to_owned(), vec![tag.clone()])]);
        let groups = HashMap::from([("shared".to_owned(), vec![group.clone()])]);
        let errors = check(&tags, &groups);

        // Ensure the diagnostic identifies both conflicting directives.
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains(&tag.to_string()));
        assert!(errors[0].contains(&group.to_string()));
    }
}
