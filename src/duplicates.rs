use std::collections::HashMap;

// This function checks that all the vectors in tags_map have a single element.
// If that condition holds, a map from label to tag is returned. Otherwise, an
// error message is returned instead.
pub fn check(
  tags_map: &HashMap<String, Vec<super::label::Label>>
) -> Result<HashMap<String, super::label::Label>, String> {
  let mut unique_tags = HashMap::new();
  let mut error = String::new();
  let mut dupes_found = false;

  for (label, tags) in tags_map {
    if tags.len() > 1 {
      dupes_found = true;
      error.push_str(
        &format!("Duplicate tags found for label `{}`:\n", label)
      );

      for tag in tags {
        error.push_str(&format!("  {}\n", tag));
      }
    }

    for tag in tags {
      unique_tags.insert(label.clone(), tag.clone());
    }
  }

  if dupes_found {
    Err(error.trim().to_string())
  } else {
    Ok(unique_tags)
  }
}

#[cfg(test)]
mod tests {
  use duplicates::check;
  use label::{Label, LabelType};
  use std::collections::HashMap;

  #[test]
  fn check_empty() {
    let tags_map = HashMap::new();
    match check(&tags_map) {
      Ok(tags) => {
        assert!(tags.is_empty());
      },
      Err(error) => {
        panic!(error);
      }
    };
  }

  #[test]
  fn check_no_dupes() {
    let mut tags_map = HashMap::new();

    let tags_vec1 = vec![
      Label {
        label_type: LabelType::Tag,
        label: "label1".to_string(),
        path: "file1.rs".to_string(),
        line_number: 1,
      },
    ];

    let tags_vec2 = vec![
      Label {
        label_type: LabelType::Tag,
        label: "label2".to_string(),
        path: "file2.rs".to_string(),
        line_number: 2,
      },
    ];

    tags_map.insert("label1".to_string(), tags_vec1);
    tags_map.insert("label2".to_string(), tags_vec2);

    match check(&tags_map) {
      Ok(tags) => {
        assert_eq!(tags.len(), 2);
      },
      Err(error) => {
        panic!(error);
      },
    };
  }

  #[test]
  fn check_dupes() {
    let mut tags_map = HashMap::new();

    let tags_vec = vec![
      Label {
        label_type: LabelType::Tag,
        label: "label".to_string(),
        path: "file1.rs".to_string(),
        line_number: 1,
      },
      Label {
        label_type: LabelType::Tag,
        label: "label".to_string(),
        path: "file2.rs".to_string(),
        line_number: 2,
      },
    ];

    tags_map.insert("label".to_string(), tags_vec.clone());

    match check(&tags_map) {
      Ok(_) => {
        panic!("The check(...) call should have failed.");
      },
      Err(error) => {
        assert!(error.contains(&format!("{}", tags_vec[0])));
        assert!(error.contains(&format!("{}", tags_vec[1])));
      },
    };
  }
}
