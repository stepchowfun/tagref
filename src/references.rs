use crate::directive::Directive;
use std::collections::HashSet;

// This function checks that references actually point to tags or groups. It returns a vector of
// error strings.
pub fn check(targets: &HashSet<String>, refs: &[Directive]) -> Vec<String> {
    let mut errors = Vec::<String>::new();

    for r#ref in refs {
        if !targets.contains(&r#ref.label) {
            errors.push(format!("No tag or group found for {ref}."));
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use crate::{
        directive::{Directive, Type},
        references::check,
    };
    use std::{collections::HashSet, path::Path};

    // This helper constructs a reference for the validation tests.
    fn reference(label: &str, line_number: usize) -> Directive {
        Directive {
            r#type: Type::Ref,
            label: label.to_owned(),
            path: Path::new("file.rs").to_owned(),
            line_number,
        }
    }

    #[test]
    fn check_empty() {
        // Ensure an empty collection has no dangling references.
        assert!(check(&HashSet::new(), &[]).is_empty());
    }

    #[test]
    fn check_tag_or_group() {
        // Construct references to both supported target types.
        let targets = HashSet::from(["tag".to_owned(), "group".to_owned()]);
        let refs = vec![reference("tag", 1), reference("group", 2)];

        // Ensure both references resolve.
        assert!(check(&targets, &refs).is_empty());
    }

    #[test]
    fn check_missing() {
        // Construct a reference with no matching target.
        let missing = reference("missing", 1);
        let errors = check(&HashSet::new(), std::slice::from_ref(&missing));

        // Ensure the diagnostic identifies the dangling reference.
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains(&missing.to_string()));
    }
}
