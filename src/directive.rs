use {
    regex::{escape, Regex},
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
    File,
    Dir,
}

#[derive(Clone, Debug)]
pub struct Directive {
    pub r#type: Type,
    pub label: String,
    pub path: PathBuf,
    pub line_number: usize,
}

// Sometimes we need to be able to print a directive.
impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}] @ {}:{}",
            match self.r#type {
                Type::Tag => "tag",
                Type::Ref => "ref",
                Type::File => "file",
                Type::Dir => "dir",
            },
            self.label,
            self.path.to_string_lossy(),
            self.line_number,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Directives {
    pub tags: Vec<Directive>,
    pub refs: Vec<Directive>,
    pub files: Vec<Directive>,
    pub dirs: Vec<Directive>,
}

// This function compiles a regular expression for matching a directive.
pub fn compile_directive_regex(sigil: &str) -> Regex {
    Regex::new(&format!(
        "(?i)\\[\\s*{}\\s*:\\s*([^\\]]*?)\\s*\\]",
        escape(sigil),
    ))
    .unwrap() // Safe by manual inspection
}

// This function returns all the directives in a file for a given type.
pub fn parse<R: BufRead>(
    tag_regex: &Regex,
    ref_regex: &Regex,
    file_regex: &Regex,
    dir_regex: &Regex,
    path: &Path,
    reader: R,
) -> Directives {
    let mut tags: Vec<Directive> = Vec::new();
    let mut refs: Vec<Directive> = Vec::new();
    let mut files: Vec<Directive> = Vec::new();
    let mut dirs: Vec<Directive> = Vec::new();

    for (line_number, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            // Tags
            for captures in tag_regex.captures_iter(&line) {
                // If we got a match, then `captures.get(1)` is guaranteed to return a `Some`. Hence
                // we are justified in unwrapping.
                tags.push(Directive {
                    r#type: Type::Tag,
                    label: captures.get(1).unwrap().as_str().to_owned(),
                    path: path.to_owned(),
                    line_number: line_number + 1,
                });
            }

            // Refs
            for captures in ref_regex.captures_iter(&line) {
                // If we got a match, then `captures.get(1)` is guaranteed to return a `Some`. Hence
                // we are justified in unwrapping.
                refs.push(Directive {
                    r#type: Type::Ref,
                    label: captures.get(1).unwrap().as_str().to_owned(),
                    path: path.to_owned(),
                    line_number: line_number + 1,
                });
            }

            // Files
            for captures in file_regex.captures_iter(&line) {
                // If we got a match, then `captures.get(1)` is guaranteed to return a `Some`. Hence
                // we are justified in unwrapping.
                files.push(Directive {
                    r#type: Type::File,
                    label: captures.get(1).unwrap().as_str().to_owned(),
                    path: path.to_owned(),
                    line_number: line_number + 1,
                });
            }

            // Directories
            for captures in dir_regex.captures_iter(&line) {
                // If we got a match, then `captures.get(1)` is guaranteed to return a `Some`. Hence
                // we are justified in unwrapping.
                dirs.push(Directive {
                    r#type: Type::Dir,
                    label: captures.get(1).unwrap().as_str().to_owned(),
                    path: path.to_owned(),
                    line_number: line_number + 1,
                });
            }
        }
    }

    Directives {
        tags,
        refs,
        files,
        dirs,
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::directive::{compile_directive_regex, parse, Type},
        std::path::Path,
    };

    #[test]
    fn parse_empty() {
        let path = Path::new("file.rs").to_owned();
        let contents = b"" as &[u8];

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents,
        );

        assert!(directives.tags.is_empty());
        assert!(directives.refs.is_empty());
        assert!(directives.files.is_empty());
        assert!(directives.dirs.is_empty());
    }

    #[test]
    fn parse_tag_basic() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?tag:label]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert_eq!(directives.tags.len(), 1);
        assert_eq!(directives.tags[0].r#type, Type::Tag);
        assert_eq!(directives.tags[0].label, "label");
        assert_eq!(directives.tags[0].path, path);
        assert_eq!(directives.tags[0].line_number, 1);
        assert!(directives.refs.is_empty());
        assert!(directives.files.is_empty());
        assert!(directives.dirs.is_empty());
    }

    #[test]
    fn parse_ref_basic() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?ref:label]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert!(directives.tags.is_empty());
        assert_eq!(directives.refs.len(), 1);
        assert_eq!(directives.refs[0].r#type, Type::Ref);
        assert_eq!(directives.refs[0].label, "label");
        assert_eq!(directives.refs[0].path, path);
        assert_eq!(directives.refs[0].line_number, 1);
        assert!(directives.files.is_empty());
        assert!(directives.dirs.is_empty());
    }

    #[test]
    fn parse_file_basic() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?file:foo/bar/baz.txt]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert!(directives.tags.is_empty());
        assert!(directives.refs.is_empty());
        assert_eq!(directives.files.len(), 1);
        assert_eq!(directives.files[0].r#type, Type::File);
        assert_eq!(directives.files[0].label, "foo/bar/baz.txt");
        assert_eq!(directives.files[0].path, path);
        assert_eq!(directives.files[0].line_number, 1);
        assert!(directives.dirs.is_empty());
    }

    #[test]
    fn parse_dir_basic() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?dir:foo/bar/baz]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert!(directives.tags.is_empty());
        assert!(directives.refs.is_empty());
        assert!(directives.files.is_empty());
        assert_eq!(directives.dirs.len(), 1);
        assert_eq!(directives.dirs[0].r#type, Type::Dir);
        assert_eq!(directives.dirs[0].label, "foo/bar/baz");
        assert_eq!(directives.dirs[0].path, path);
        assert_eq!(directives.dirs[0].line_number, 1);
    }

    #[test]
    fn parse_multiple_per_line() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?tag:label][?ref:label][?file:foo/bar/baz.txt][?dir:foo/bar/baz]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert_eq!(directives.tags.len(), 1);
        assert_eq!(directives.tags[0].r#type, Type::Tag);
        assert_eq!(directives.tags[0].label, "label");
        assert_eq!(directives.tags[0].path, path);
        assert_eq!(directives.tags[0].line_number, 1);

        assert_eq!(directives.refs.len(), 1);
        assert_eq!(directives.refs[0].r#type, Type::Ref);
        assert_eq!(directives.refs[0].label, "label");
        assert_eq!(directives.refs[0].path, path);
        assert_eq!(directives.refs[0].line_number, 1);

        assert_eq!(directives.files.len(), 1);
        assert_eq!(directives.files[0].r#type, Type::File);
        assert_eq!(directives.files[0].label, "foo/bar/baz.txt");
        assert_eq!(directives.files[0].path, path);
        assert_eq!(directives.files[0].line_number, 1);

        assert_eq!(directives.dirs.len(), 1);
        assert_eq!(directives.dirs[0].r#type, Type::Dir);
        assert_eq!(directives.dirs[0].label, "foo/bar/baz");
        assert_eq!(directives.dirs[0].path, path);
        assert_eq!(directives.dirs[0].line_number, 1);
    }

    #[test]
    fn parse_multiple_lines() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?tag:label]
      [?ref:label]
      [?file:foo/bar/baz.txt]
      [?dir:foo/bar/baz]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert_eq!(directives.tags.len(), 1);
        assert_eq!(directives.tags[0].r#type, Type::Tag);
        assert_eq!(directives.tags[0].label, "label");
        assert_eq!(directives.tags[0].path, path);
        assert_eq!(directives.tags[0].line_number, 1);

        assert_eq!(directives.refs.len(), 1);
        assert_eq!(directives.refs[0].r#type, Type::Ref);
        assert_eq!(directives.refs[0].label, "label");
        assert_eq!(directives.refs[0].path, path);
        assert_eq!(directives.refs[0].line_number, 2);

        assert_eq!(directives.files.len(), 1);
        assert_eq!(directives.files[0].r#type, Type::File);
        assert_eq!(directives.files[0].label, "foo/bar/baz.txt");
        assert_eq!(directives.files[0].path, path);
        assert_eq!(directives.files[0].line_number, 3);

        assert_eq!(directives.dirs.len(), 1);
        assert_eq!(directives.dirs[0].r#type, Type::Dir);
        assert_eq!(directives.dirs[0].label, "foo/bar/baz");
        assert_eq!(directives.dirs[0].path, path);
        assert_eq!(directives.dirs[0].line_number, 4);
    }

    #[test]
    fn parse_whitespace() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [  ?tag   :  foo  bar               ]
      [  ?ref   :  foo  bar               ]
      [  ?file  :  foo  bar/baz  qux.txt  ]
      [  ?dir   :  foo  bar/baz  qux      ]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert_eq!(directives.tags.len(), 1);
        assert_eq!(directives.tags[0].r#type, Type::Tag);
        assert_eq!(directives.tags[0].label, "foo  bar");
        assert_eq!(directives.tags[0].path, path);
        assert_eq!(directives.tags[0].line_number, 1);

        assert_eq!(directives.refs.len(), 1);
        assert_eq!(directives.refs[0].r#type, Type::Ref);
        assert_eq!(directives.refs[0].label, "foo  bar");
        assert_eq!(directives.refs[0].path, path);
        assert_eq!(directives.refs[0].line_number, 2);

        assert_eq!(directives.files.len(), 1);
        assert_eq!(directives.files[0].r#type, Type::File);
        assert_eq!(directives.files[0].label, "foo  bar/baz  qux.txt");
        assert_eq!(directives.files[0].path, path);
        assert_eq!(directives.files[0].line_number, 3);

        assert_eq!(directives.dirs.len(), 1);
        assert_eq!(directives.dirs[0].r#type, Type::Dir);
        assert_eq!(directives.dirs[0].label, "foo  bar/baz  qux");
        assert_eq!(directives.dirs[0].path, path);
        assert_eq!(directives.dirs[0].line_number, 4);
    }

    #[test]
    fn parse_case() {
        let path = Path::new("file.rs").to_owned();
        let contents = r"
      [?tag:label]
      [?TAG:LABEL]
      [?ref:label]
      [?REF:LABEL]
      [?file:foo/bar/baz.txt]
      [?FILE:FOO/BAR/BAZ.TXT]
      [?dir:foo/bar/baz]
      [?DIR:FOO/BAR/BAZ]
    "
        .trim()
        .replace('?', "")
        .as_bytes()
        .to_owned();

        let tag_regex = compile_directive_regex("tag");
        let ref_regex = compile_directive_regex("ref");
        let file_regex = compile_directive_regex("file");
        let dir_regex = compile_directive_regex("dir");

        let directives = parse(
            &tag_regex,
            &ref_regex,
            &file_regex,
            &dir_regex,
            &path,
            contents.as_ref(),
        );

        assert_eq!(directives.tags.len(), 2);
        assert_eq!(directives.tags[0].r#type, Type::Tag);
        assert_eq!(directives.tags[0].label, "label");
        assert_eq!(directives.tags[0].path, path);
        assert_eq!(directives.tags[0].line_number, 1);
        assert_eq!(directives.tags[1].r#type, Type::Tag);
        assert_eq!(directives.tags[1].label, "LABEL");
        assert_eq!(directives.tags[1].path, path);
        assert_eq!(directives.tags[1].line_number, 2);

        assert_eq!(directives.refs.len(), 2);
        assert_eq!(directives.refs[0].r#type, Type::Ref);
        assert_eq!(directives.refs[0].label, "label");
        assert_eq!(directives.refs[0].path, path);
        assert_eq!(directives.refs[0].line_number, 3);
        assert_eq!(directives.refs[1].r#type, Type::Ref);
        assert_eq!(directives.refs[1].label, "LABEL");
        assert_eq!(directives.refs[1].path, path);
        assert_eq!(directives.refs[1].line_number, 4);

        assert_eq!(directives.files.len(), 2);
        assert_eq!(directives.files[0].r#type, Type::File);
        assert_eq!(directives.files[0].label, "foo/bar/baz.txt");
        assert_eq!(directives.files[0].path, path);
        assert_eq!(directives.files[0].line_number, 5);
        assert_eq!(directives.files[1].r#type, Type::File);
        assert_eq!(directives.files[1].label, "FOO/BAR/BAZ.TXT");
        assert_eq!(directives.files[1].path, path);
        assert_eq!(directives.files[1].line_number, 6);

        assert_eq!(directives.dirs.len(), 2);
        assert_eq!(directives.dirs[0].r#type, Type::Dir);
        assert_eq!(directives.dirs[0].label, "foo/bar/baz");
        assert_eq!(directives.dirs[0].path, path);
        assert_eq!(directives.dirs[0].line_number, 7);
        assert_eq!(directives.dirs[1].r#type, Type::Dir);
        assert_eq!(directives.dirs[1].label, "FOO/BAR/BAZ");
        assert_eq!(directives.dirs[1].path, path);
        assert_eq!(directives.dirs[1].line_number, 8);
    }
}
