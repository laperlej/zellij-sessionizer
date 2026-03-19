use std::collections::BTreeMap;
use std::path::PathBuf;

use zellij_tile::prelude::LayoutInfo;

use crate::ROOT;

#[derive(Debug)]
pub struct Config {
    pub dirs: Vec<PathBuf>,
    pub layout: LayoutInfo,
    pub marker_files: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dirs: vec![PathBuf::from(ROOT)],
            layout: LayoutInfo::BuiltIn("default".to_string()),
            marker_files: None,
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
    return dirs.split(';').map(PathBuf::from).collect();
}

fn parse_marker_files(markers: &str) -> Vec<String> {
    markers.split(';').map(|s| s.trim().to_string()).collect()
}

impl From<BTreeMap<String, String>> for Config {
    fn from(config: BTreeMap<String, String>) -> Self {
        let dirs: Vec<PathBuf> = match config.get("root_dirs") {
            Some(root_dirs) => parse_dirs(root_dirs),
            _ => vec![PathBuf::from(ROOT)],
        };
        let layout = match config.get("session_layout") {
            Some(layout) => parse_layout(layout),
            _ => LayoutInfo::BuiltIn("default".to_string()),
        };
        let marker_files = config.get("marker_files").map(|m| parse_marker_files(m));
        Self {
            dirs,
            layout,
            marker_files,
        }
    }
}
