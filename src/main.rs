use zellij_tile::prelude::*;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::path::Path;

mod textinput;
mod dirlist;
use textinput::TextInput;
use dirlist::DirList;

const ROOT: &str = "/host";

#[derive(Debug, Default)]
struct State {
    dirlist: DirList,
    cwd: PathBuf,
    textinput: TextInput,
    dirs: Vec<PathBuf>,

    debug: String,
}

register_plugin!(State);

fn switch_session_with_cwd(dir: &Path) {
    let session_name = dir.file_name().unwrap().to_str().unwrap();
    let layout = LayoutInfo::File(String::from("default"));
    let cwd = dir.to_path_buf();
    switch_session_with_layout(Some(session_name), layout, Some(cwd));
}

impl State {
    fn change_root(&mut self, path: &Path) -> PathBuf { 
        self.cwd.join(path.strip_prefix(ROOT).unwrap())
    }

    fn make_dirlist(&mut self, paths: &[(PathBuf, Option<FileMetadata>)]) -> Vec<String> {
        paths
            .iter()
            .filter(|(p, _)| { p.is_dir() && !is_hidden(p) })
            .map(|(p, _)| {
                if p.starts_with(ROOT) {
                    self.change_root(p)
                } else {
                    p.to_path_buf()
                }
            })
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    }
}


impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.cwd = get_plugin_ids().initial_cwd;
        match configuration.get("root_dirs") {
            Some(root_dirs) => {
                self.dirs = root_dirs.split(';').map(PathBuf::from).collect();
            }
            _ => {
                self.dirs = vec![PathBuf::from(ROOT)];
            }
        }

        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);
        subscribe(&[
            EventType::Key,
            EventType::FileSystemUpdate,
        ]);
        self.dirlist.reset();
        self.textinput.reset();
        let host = PathBuf::from(ROOT);
        for dir in &self.dirs {
            let relative_path = match dir.strip_prefix(self.cwd.as_path()) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let host_path = host.join(relative_path);
            scan_host_folder(&host_path);
        }
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::FileSystemUpdate(paths) => {
                should_render = true;
                let dirs = self.make_dirlist(&paths);
                self.dirlist.update_dirs(dirs);
            },
            Event::Key(key) => {
                should_render = true;
                match key {
                    Key::Esc => {
                        hide_self();
                    }
                    Key::Down => {
                        self.dirlist.handle_down();
                    }
                    Key::Up => {
                        self.dirlist.handle_up();
                    }
                    Key::Char('\n') | Key::Char('\r') => {
                        if let Some(selected) = self.dirlist.get_selected() {
                            switch_session_with_cwd(Path::new(&selected));
                            hide_self();
                        }
                    }
                    Key::Backspace => {
                        self.textinput.handle_backspace();
                        self.dirlist.set_search_term(self.textinput.get_text().as_str());
                    }
                    Key::Char(c) => {
                        self.textinput.handle_char(c);
                        self.dirlist.set_search_term(self.textinput.get_text().as_str());
                    }
                    _ => (),
                }
            }
            _ => (),
        };
        should_render 
    }

    fn render(&mut self, rows: usize, cols: usize) {
        println!();
        self.dirlist.render(rows.saturating_sub(5), cols);
        println!();
        println!();
        self.textinput.render(rows, cols);
        println!();
        println!("{}", self.debug);
    }
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
