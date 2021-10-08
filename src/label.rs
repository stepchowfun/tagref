use {
    regex::Regex,
    std::{
        fmt,
        io::BufRead,
        path::{Path, PathBuf},
    },
};

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
    pub line_number: usize,
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
            self.line_number,
        )
    }
}

// This function returns all the labels in a file for a given type.
pub fn parse<R: BufRead>(
    tag_regex: &Regex,
    ref_regex: &Regex,
    label_type: Type,
    path: &Path,
    reader: R,
) -> Vec<Label> {
    let regex = match label_type {
        Type::Tag => tag_regex,
        Type::Ref => ref_regex,
    };

    let mut labels: Vec<Label> = Vec::new();

    for (line_number, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            for captures in regex.captures_iter(&line) {
                // If we got a match, then `captures.get(1)` is guaranteed to return a `Some`. Hence
                // we are justified in unwrapping.
                labels.push(Label {
                    label_type,
                    label: captures.get(1).unwrap().as_str().to_owned(),
                    path: path.to_owned(),
                    line_number: line_number + 1,
                });
            }
        }
    }

    labels
}

#[cfg(test)]
mod tests {
    use {
        crate::label::{parse, Type},
        regex::Regex,
        std::path::Path,
    };

    const TAG_REGEX: &str = "(?i)\\[\\s*tag\\s*:\\s*([^\\]\\s]*)\\s*\\]";
    const REF_REGEX: &str = "(?i)\\[\\s*ref\\s*:\\s*([^\\]\\s]*)\\s*\\]";

    #[test]
    fn parse_empty() {
        let path = Path::new("file.rs").to_owned();
        let contents = b"" as &[u8];

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let tags = parse(&tag_regex, &ref_regex, Type::Tag, &path, contents);

        assert!(tags.is_empty());
    }

    #[test]
    fn parse_tag_basic() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [tag:label1]
    "
        .trim()
        .as_bytes();

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let tags = parse(&tag_regex, &ref_regex, Type::Tag, &path, contents);

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
        .as_bytes();

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let refs = parse(&tag_regex, &ref_regex, Type::Ref, &path, contents);

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
        .as_bytes();

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let tags = parse(&tag_regex, &ref_regex, Type::Tag, &path, contents);

        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].label_type, Type::Tag);
        assert_eq!(tags[0].label, "label2");
        assert_eq!(tags[0].path, path);
        assert_eq!(tags[0].line_number, 1);
    }

    #[test]
    fn parse_case() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [ TAG: label3 ]
      [ TAG: LABEL3 ]
    "
        .trim()
        .as_bytes();

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let tags = parse(&tag_regex, &ref_regex, Type::Tag, &path, contents);

        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].label_type, Type::Tag);
        assert_eq!(tags[0].label, "label3");
        assert_eq!(tags[0].path, path);
        assert_eq!(tags[0].line_number, 1);
        assert_eq!(tags[1].label_type, Type::Tag);
        assert_eq!(tags[1].label, "LABEL3");
        assert_eq!(tags[1].path, path);
        assert_eq!(tags[1].line_number, 2);
        assert_ne!(tags[0].label, tags[1].label);
    }

    #[test]
    fn parse_multiple_per_line() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [tag:label4][tag:label5]
    "
        .trim()
        .as_bytes();

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let tags = parse(&tag_regex, &ref_regex, Type::Tag, &path, contents);

        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].label_type, Type::Tag);
        assert_eq!(tags[0].label, "label4");
        assert_eq!(tags[0].path, path);
        assert_eq!(tags[0].line_number, 1);
        assert_eq!(tags[1].label_type, Type::Tag);
        assert_eq!(tags[1].label, "label5");
        assert_eq!(tags[1].path, path);
        assert_eq!(tags[1].line_number, 1);
    }

    #[test]
    fn parse_multiple_lines() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [tag:label6]
      [tag:label7]
    "
        .trim()
        .as_bytes();

        let tag_regex: Regex = Regex::new(TAG_REGEX).unwrap();
        let ref_regex: Regex = Regex::new(REF_REGEX).unwrap();

        let tags = parse(&tag_regex, &ref_regex, Type::Tag, &path, contents);

        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].label_type, Type::Tag);
        assert_eq!(tags[0].label, "label6");
        assert_eq!(tags[0].path, path);
        assert_eq!(tags[0].line_number, 1);
        assert_eq!(tags[1].label_type, Type::Tag);
        assert_eq!(tags[1].label, "label7");
        assert_eq!(tags[1].path, path);
        assert_eq!(tags[1].line_number, 2);
    }
}
