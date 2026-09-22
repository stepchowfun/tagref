use crate::directive::Directive;
use std::{collections::HashMap, fmt::Write};

// This function checks that every group has at least two members. It returns a vector of error
// strings.
pub fn check(groups: &HashMap<String, Vec<Directive>>) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for (label, members) in groups {
        if members.len() < 2 {
            let mut error = String::new();
            let _ = writeln!(error, "Group `{label}` has only one member:");
            for member in members {
                let _ = writeln!(error, "  {member}");
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
        groups::check,
    };
    use std::{collections::HashMap, path::Path};

    // This helper constructs a group member for the validation tests.
    fn member(label: &str, line_number: usize) -> Directive {
        Directive {
            r#type: Type::Group,
            label: label.to_owned(),
            path: Path::new("file.rs").to_owned(),
            line_number,
        }
    }

    #[test]
    fn check_empty() {
        // Ensure an empty collection has no invalid groups.
        assert!(check(&HashMap::new()).is_empty());
    }

    #[test]
    fn check_valid() {
        // Construct a group containing the required two members.
        let groups = HashMap::from([(
            "group".to_owned(),
            vec![member("group", 1), member("group", 2)],
        )]);

        // Ensure the multi-member group is valid.
        assert!(check(&groups).is_empty());
    }

    #[test]
    fn check_singleton() {
        // Construct a group with only one member.
        let singleton = member("group", 1);
        let groups = HashMap::from([("group".to_owned(), vec![singleton.clone()])]);
        let errors = check(&groups);

        // Ensure the diagnostic identifies both the group and its member.
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Group `group` has only one member"));
        assert!(errors[0].contains(&singleton.to_string()));
    }
}
