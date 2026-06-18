use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

// The name of the config file Tagref searches for
const CONFIG_FILE_NAME: &str = "tagref.yml";

// The default sigil for tag declarations
const DEFAULT_TAG_SIGIL: &str = "tag";

// The default sigil for tag references
const DEFAULT_REF_SIGIL: &str = "ref";

// The default sigil for file references
const DEFAULT_FILE_SIGIL: &str = "file";

// The default sigil for directory references
const DEFAULT_DIR_SIGIL: &str = "dir";

// The resolved configuration used by Tagref at runtime
#[derive(Debug)]
pub struct Config {
    pub project_root: PathBuf,
    pub tag_sigil: String,
    pub ref_sigil: String,
    pub file_sigil: String,
    pub dir_sigil: String,
    pub ignore_rules: Vec<String>,
}

// The on-disk `tagref.yml` schema
#[allow(clippy::struct_field_names)]
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    tag_sigil: Option<String>,
    ref_sigil: Option<String>,
    file_sigil: Option<String>,
    dir_sigil: Option<String>,
    ignore_rules: Option<Vec<String>>,
}

// Loads the configuration, either from an explicit config path or by searching for one
pub fn load(config_path: Option<&Path>) -> Result<Config, String> {
    let invocation_dir =
        env::current_dir().map_err(|error| format!("Error getting current directory: {error}"))?;

    if let Some(config_path) = config_path {
        let config_path = if config_path.is_absolute() {
            config_path.to_owned()
        } else {
            invocation_dir.join(config_path)
        };
        if !config_path.is_file() {
            return Err(format!(
                "No config file found at {}.",
                config_path.to_string_lossy(),
            ));
        }
        return load_from_file(&config_path);
    }

    if let Some(config_path) = find_config(&invocation_dir) {
        load_from_file(&config_path)
    } else {
        Ok(Config {
            project_root: invocation_dir,
            tag_sigil: DEFAULT_TAG_SIGIL.to_owned(),
            ref_sigil: DEFAULT_REF_SIGIL.to_owned(),
            file_sigil: DEFAULT_FILE_SIGIL.to_owned(),
            dir_sigil: DEFAULT_DIR_SIGIL.to_owned(),
            ignore_rules: Vec::new(),
        })
    }
}

// Finds the nearest config file at or above `start`
fn find_config(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .map(|ancestor| ancestor.join(CONFIG_FILE_NAME))
        .find(|path| path.is_file())
}

// Parses a config file and applies defaults
fn load_from_file(path: &Path) -> Result<Config, String> {
    let project_root = path
        .parent()
        .ok_or_else(|| format!("Config file {} has no parent.", path.to_string_lossy()))?
        .to_owned();
    let contents = fs::read_to_string(path).map_err(|error| {
        format!(
            "Error reading config file {}: {error}",
            path.to_string_lossy(),
        )
    })?;
    let raw_config = if contents.trim().is_empty() {
        RawConfig::default()
    } else {
        yaml_serde::from_str::<RawConfig>(&contents).map_err(|error| {
            format!(
                "Error parsing config file {}: {error}",
                path.to_string_lossy(),
            )
        })?
    };
    let ignore_rules = raw_config.ignore_rules.unwrap_or_default();
    for ignore_rule in &ignore_rules {
        if ignore_rule.starts_with('!') {
            return Err(format!(
                "Invalid ignore rule `{ignore_rule}` in {}: ignore_rules only supports exclusions.",
                path.to_string_lossy(),
            ));
        }
    }

    Ok(Config {
        project_root,
        tag_sigil: raw_config
            .tag_sigil
            .unwrap_or_else(|| DEFAULT_TAG_SIGIL.to_owned()),
        ref_sigil: raw_config
            .ref_sigil
            .unwrap_or_else(|| DEFAULT_REF_SIGIL.to_owned()),
        file_sigil: raw_config
            .file_sigil
            .unwrap_or_else(|| DEFAULT_FILE_SIGIL.to_owned()),
        dir_sigil: raw_config
            .dir_sigil
            .unwrap_or_else(|| DEFAULT_DIR_SIGIL.to_owned()),
        ignore_rules,
    })
}
