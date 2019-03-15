use std::collections::HashMap;

// This function checks that references actually point to tags. If a missing
// tag is encountered, an error message is returned.
pub fn check(
  tags: &HashMap<String, super::label::Label>,
  references: &[super::label::Label]
) -> Option<String> {
  let mut error = String::new();
  let mut missing_tags = false;

  for reference in references {
    if !tags.contains_key(&reference.label) {
      missing_tags = true;
      error.push_str(&format!("No tag found for {}.\n", reference));
    }
  }

  if missing_tags {
    Some(error.trim().to_string())
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use crate::citations::check;
  use crate::label::{Label, Type};
  use std::collections::HashMap;

  #[test]
  fn check_empty() {
    let tags = HashMap::new();
    let references = vec![];

    match check(&tags, &references) {
      None => (),
      Some(error) => {
        panic!(error);
      },
    };
  }

  #[test]
  fn check_ok() {
    let mut tags = HashMap::new();
    tags.insert(
      "label1".to_string(),
      Label {
        label_type: Type::Tag,
        label: "label1".to_string(),
        path: "file1.rs".to_string(),
        line_number: 1,
      }
    );

    let references = vec![
      Label {
        label_type: Type::Ref,
        label: "label1".to_string(),
        path: "file1.rs".to_string(),
        line_number: 1,
      },
    ];

    match check(&tags, &references) {
      None => (),
      Some(error) => {
        panic!(error);
      },
    };
  }

  #[test]
  fn check_missing() {
    let mut tags = HashMap::new();
    tags.insert(
      "label1".to_string(),
      Label {
        label_type: Type::Tag,
        label: "label1".to_string(),
        path: "file1.rs".to_string(),
        line_number: 1,
      }
    );

    let references = vec![
      Label {
        label_type: Type::Ref,
        label: "label1".to_string(),
        path: "file1.rs".to_string(),
        line_number: 1,
      },
      Label {
        label_type: Type::Ref,
        label: "label2".to_string(),
        path: "file2.rs".to_string(),
        line_number: 2,
      },
    ];

    match check(&tags, &references) {
      None => {
        panic!("The check(...) call should have failed.");
      },
      Some(error) => {
        assert!(error.contains(&format!("{}", references[1].label)));
      },
    };
  }
}
