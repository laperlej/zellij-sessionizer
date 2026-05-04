use zellij_tile::prelude::*;

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use config::Config;

mod config;
mod dirlist;
mod filter;
mod textinput;
use dirlist::DirList;
use textinput::TextInput;

const ROOT: &str = "/host";

#[derive(Debug, Default)]
struct State {
    dirlist: DirList,
    cwd: PathBuf,
    textinput: TextInput,
    current_session: String,

    config: Config,
    debug: String,
    rows: usize,
}

fn matches_key(
    key: &KeyWithModifier,
    bare_key: BareKey,
    modifiers: Option<&[KeyModifier]>,
) -> bool {
    if key.bare_key != bare_key {
        return false;
    }
    modifiers.map_or(key.key_modifiers.is_empty(), |mods| key.has_modifiers(mods))
}

register_plugin!(State);

impl State {
    fn change_root(&mut self, path: &Path) -> PathBuf {
        self.cwd.join(path.strip_prefix(ROOT).unwrap())
    }

    fn switch_session_with_cwd(&self, dir: &Path) -> Result<(), String> {
        let session_name = dir.file_name().unwrap().to_str().unwrap();
        let cwd = dir.to_path_buf();
        // strip_prefix("/") only works for Unix absolute paths; on Windows dirs
        // start with a drive letter so we fall back to the configured layout.
        let layout = match dir.strip_prefix("/") {
            Ok(relative) => {
                let host_layout_path = PathBuf::from(ROOT).join(relative).join("layout.kdl");
                if host_layout_path.exists() {
                    LayoutInfo::File(host_layout_path.to_str().unwrap().into())
                } else {
                    self.config.layout.clone()
                }
            }
            Err(_) => self.config.layout.clone(),
        };
        // Switch session will panic if the session is the current session
        if session_name != self.current_session {
            switch_session_with_layout(Some(session_name), layout, Some(cwd));
        }
        Ok(())
    }

    fn make_dirlist(&mut self, paths: &[(PathBuf, Option<FileMetadata>)]) -> Vec<String> {
        paths
            .iter()
            .filter(|(p, metadata)| {
                // Use Zellij's FileMetadata when available instead of p.is_dir(),
                // because inside the WASM sandbox on Windows the virtual /host/...
                // paths cannot be stat'd by the OS and is_dir() always returns false.
                let is_dir = metadata.map(|m| m.is_dir).unwrap_or_else(|| p.is_dir());
                is_dir && !is_hidden(p)
            })
            .map(|(p, _)| {
                // On Windows, Zellij returns paths with backslashes in FileSystemUpdate
                // (e.g. /host\Users\azin\repos\project). Inside the WASM sandbox, which
                // uses Unix path semantics, backslashes are not separators and are treated
                // as part of the filename, breaking starts_with("/host"). Normalise first.
                let normalized = PathBuf::from(p.to_string_lossy().replace('\\', "/"));
                if normalized.starts_with(ROOT) {
                    self.change_root(&normalized)
                } else {
                    normalized
                }
            })
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
}

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.cwd = get_plugin_ids().initial_cwd;
        self.config = Config::from(configuration);
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
        subscribe(&[
            EventType::Key,
            EventType::FileSystemUpdate,
            EventType::SessionUpdate,
        ]);
        self.dirlist.reset();
        self.textinput.reset();
        let host = PathBuf::from(ROOT);
        for dir in &self.config.root_dirs {
            let relative_path = match strip_prefix_portable(dir, self.cwd.as_path()) {
                Some(p) => p,
                None => continue,
            };
            let host_path = host.join(relative_path);
            scan_host_folder(&host_path);
        }
        let individual_dirs: Vec<String> = self
            .config
            .individual_dirs
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        self.dirlist.update_dirs(individual_dirs);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::FileSystemUpdate(paths) => {
                let dirs = self.make_dirlist(&paths);
                self.dirlist.update_dirs(dirs);
                should_render = true;
            }
            Event::SessionUpdate(sessions, _) => {
                for session in sessions.iter() {
                    if session.is_current_session {
                        self.current_session = session.name.clone();
                        break;
                    }
                }
                should_render = true;
            }
            Event::Key(key) => {
                should_render = true;
                match key {
                    k if matches_key(&k, BareKey::Char('d'), Some(&[KeyModifier::Ctrl]))
                        || matches_key(&k, BareKey::Esc, None) =>
                    {
                        close_self()
                    }

                    k if matches_key(&k, BareKey::Up, None)
                        || matches_key(&k, BareKey::Tab, Some(&[KeyModifier::Shift]))
                        || matches_key(&k, BareKey::Char('p'), Some(&[KeyModifier::Ctrl])) =>
                    {
                        self.dirlist.handle_up();
                    }

                    k if matches_key(&k, BareKey::Down, None)
                        || matches_key(&k, BareKey::Tab, None)
                        || matches_key(&k, BareKey::Char('n'), Some(&[KeyModifier::Ctrl])) =>
                    {
                        self.dirlist.handle_down();
                    }

                    k if matches_key(&k, BareKey::PageUp, None) => {
                        self.dirlist.handle_half_page_up(self.rows);
                    }

                    k if matches_key(&k, BareKey::PageDown, None) => {
                        self.dirlist.handle_half_page_down(self.rows);
                    }

                    k if matches_key(&k, BareKey::Enter, None) => {
                        if let Some(selected) = self.dirlist.get_selected() {
                            let _ = self.switch_session_with_cwd(Path::new(&selected));
                            close_self();
                        }
                    }

                    k if matches_key(&k, BareKey::Backspace, None) => {
                        self.textinput.handle_backspace();
                        self.dirlist
                            .set_search_term(self.textinput.get_text().as_str());
                    }

                    k if matches_key(&k, BareKey::Char('w'), Some(&[KeyModifier::Ctrl])) => {
                        self.textinput.handle_delete_word();
                        self.dirlist
                            .set_search_term(self.textinput.get_text().as_str());
                    }

                    KeyWithModifier {
                        bare_key: BareKey::Char(c),
                        ..
                    } => {
                        self.textinput.handle_char(c);
                        self.dirlist
                            .set_search_term(self.textinput.get_text().as_str());
                    }

                    _ => (),
                }
            }
            _ => (),
        }
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        self.rows = rows.saturating_sub(4);
        println!();
        self.dirlist.render(self.rows, cols);
        println!();
        self.textinput.render(rows, cols);
        println!();
        if !self.debug.is_empty() {
            println!();
            println!("{}", self.debug);
        }
    }
}

/// `Path::strip_prefix` is case-sensitive and treats `/` and `\` as distinct,
/// which causes silent failures on Windows where the host may mix separators
/// or use different drive-letter casing.  This helper normalises both paths to
/// lowercase with forward slashes before comparing, then slices the *original*
/// string so the returned relative path keeps its original casing.
fn strip_prefix_portable(path: &Path, prefix: &Path) -> Option<PathBuf> {
    // Fast path: stdlib works fine for pure Unix paths (no drive letters).
    if let Ok(rel) = path.strip_prefix(prefix) {
        return Some(rel.to_path_buf());
    }
    // Slow path: normalise separators and case for Windows-style paths.
    let norm = |p: &Path| p.to_string_lossy().to_lowercase().replace('\\', "/");
    let path_n = norm(path);
    let prefix_n = norm(prefix);
    // Ensure the prefix ends with `/` to avoid matching partial component names
    // (e.g. prefix "c:/foo" accidentally matching "c:/foobar").
    let prefix_sep = if prefix_n.ends_with('/') {
        prefix_n.clone()
    } else {
        format!("{}/", prefix_n)
    };
    if path_n == prefix_n {
        Some(PathBuf::new())
    } else if path_n.starts_with(&prefix_sep) {
        // Slice the *original* (un-normalised) string so casing is preserved.
        let original = path.to_string_lossy();
        Some(PathBuf::from(&original[prefix_sep.len()..]))
    } else {
        None
    }
}

fn is_hidden(path: &Path) -> bool {
    const WHITELIST: [&str; 1] = [".config"];

    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.') && !WHITELIST.contains(&s))
        .unwrap_or(false)
}
