use regex::Regex;
use std::{
  fmt,
  path::{Path, PathBuf},
};

const TAG_REGEX: &str = r"(?i)\[\s*tag\s*:([^\]]*)\]";
const REFERENCE_REGEX: &str = r"(?i)\[\s*ref\s*:([^\]]*)\]";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
  Tag,
  Ref,
}

#[derive(Clone, Debug)]
pub struct Label {
  pub label_type: Type,
  pub label: String,
  pub path: PathBuf,
  pub line_number: i64,
}

// Sometimes we need to be able to print a label.
impl fmt::Display for Label {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "[{}:{}] @ {}:{}",
      match self.label_type {
        Type::Tag => "tag",
        Type::Ref => "ref",
      },
      self.label,
      self.path.to_string_lossy(),
      self.line_number
    )
  }
}

// This function returns all the labels in a file for a given type.
pub fn parse(label_type: Type, path: &Path, contents: &str) -> Vec<Label> {
  let regex = Regex::new(match label_type {
    Type::Tag => TAG_REGEX,
    Type::Ref => REFERENCE_REGEX,
  })
  .unwrap();

  let mut labels: Vec<Label> = Vec::new();
  let mut line_number = 1;

  for line in contents.lines() {
    for captures in regex.captures_iter(line) {
      // If we got a match, then captures.get(1) is guaranteed to return a
      // Some. Hence we are justified in unwrapping.
      labels.push(Label {
        label_type,
        label: captures.get(1).unwrap().as_str().trim().to_owned(),
        path: path.to_owned(),
        line_number,
      });
    }
    line_number += 1;
  }

  labels
}

#[cfg(test)]
mod tests {
  use crate::label::{parse, Type};
  use std::path::Path;

  #[test]
  fn parse_empty() {
    let path = Path::new("file.rs").to_owned();
    let contents = String::new();

    let tags = parse(Type::Tag, &path, &contents);

    assert!(tags.is_empty());
  }

  #[test]
  fn parse_tag_basic() {
    let path = Path::new("file.rs").to_owned();
    let contents = r"
      [tag:label1]
    "
    .trim()
    .to_owned();

    let tags = parse(Type::Tag, &path, &contents);

    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].label_type, Type::Tag);
    assert_eq!(tags[0].label, "label1");
    assert_eq!(tags[0].path, path);
    assert_eq!(tags[0].line_number, 1);
  }

  #[test]
  fn parse_ref_basic() {
    let path = Path::new("file.rs").to_owned();
    let contents = r"
      [ref:label1]
    "
    .trim()
    .to_owned();

    let refs = parse(Type::Ref, &path, &contents);

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].label_type, Type::Ref);
    assert_eq!(refs[0].label, "label1");
    assert_eq!(refs[0].path, path);
    assert_eq!(refs[0].line_number, 1);
  }

  #[test]
  fn parse_whitespace() {
    let path = Path::new("file.rs").to_owned();
    let contents = r"
      [ TAG: label2 ]
    "
    .trim()
    .to_owned();

    let tags = parse(Type::Tag, &path, &contents);

    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].label_type, Type::Tag);
    assert_eq!(tags[0].label, "label2");
    assert_eq!(tags[0].path, path);
    assert_eq!(tags[0].line_number, 1);
  }

  #[test]
  fn parse_multiple_per_line() {
    let path = Path::new("file.rs").to_owned();
    let contents = r"
      [tag:label3] [tag:label4]
    "
    .trim()
    .to_owned();

    let tags = parse(Type::Tag, &path, &contents);

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].label_type, Type::Tag);
    assert_eq!(tags[0].label, "label3");
    assert_eq!(tags[0].path, path);
    assert_eq!(tags[0].line_number, 1);
    assert_eq!(tags[1].label_type, Type::Tag);
    assert_eq!(tags[1].label, "label4");
    assert_eq!(tags[1].path, path);
    assert_eq!(tags[1].line_number, 1);
  }

  #[test]
  fn parse_multiple_lines() {
    let path = Path::new("file.rs").to_owned();
    let contents = r"
      [tag:label5]
      [tag:label6]
    "
    .trim()
    .to_owned();

    let tags = parse(Type::Tag, &path, &contents);

    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].label_type, Type::Tag);
    assert_eq!(tags[0].label, "label5");
    assert_eq!(tags[0].path, path);
    assert_eq!(tags[0].line_number, 1);
    assert_eq!(tags[1].label_type, Type::Tag);
    assert_eq!(tags[1].label, "label6");
    assert_eq!(tags[1].path, path);
    assert_eq!(tags[1].line_number, 2);
  }
}
