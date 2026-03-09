use std::collections::BTreeMap;
use std::path::PathBuf;

use zellij_tile::prelude::LayoutInfo;

use crate::ROOT;

#[derive(Debug)]
pub struct Config {
    pub root_dirs: Vec<PathBuf>,
    pub individual_dirs: Vec<PathBuf>,
    pub layout: LayoutInfo,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_dirs: vec![PathBuf::from(ROOT)],
            individual_dirs: vec![],
            layout: LayoutInfo::BuiltIn("default".to_string())
        }
    }
}

fn parse_layout(layout: &str) -> LayoutInfo {
    // builtin: ":default" custom: "default"
    if layout.starts_with(":") {
        LayoutInfo::BuiltIn(layout.trim_start_matches(':').to_string())
    } else {
        LayoutInfo::File(layout.to_string())
    }
}


fn parse_dirs(dirs: &str) -> Vec<PathBuf> {
    return dirs.split(';').map(PathBuf::from).collect()
}

impl From<BTreeMap<String, String>> for Config {
    fn from(config: BTreeMap<String, String>) -> Self {
        let root_dirs: Vec<PathBuf> = match config.get("root_dirs") {
            Some(root_dirs) => parse_dirs(root_dirs),
            _ => vec![PathBuf::from(ROOT)]
        };
        let individual_dirs: Vec<PathBuf> = match config.get("individual_dirs") {
            Some(individual_dirs) => parse_dirs(individual_dirs),
            _ => vec![]
        };
        let layout = match config.get("session_layout") {
            Some(layout) => parse_layout(layout),
            _ => LayoutInfo::BuiltIn("default".to_string())
        };
        Self {
            root_dirs,
            individual_dirs,
            layout
        }
    }
}
