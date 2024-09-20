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
    textinput: TextInput,
    root_cwd: PathBuf,

    userspace_configuration: BTreeMap<String, String>,
}

register_plugin!(State);

fn switch_session_with_cwd(dir: &Path) {
    let session_name = dir.file_name().unwrap().to_str().unwrap();
    let layout = LayoutInfo::BuiltIn(String::from("default"));
    let cwd = dir.to_path_buf();
    switch_session_with_layout(Some(session_name), layout, Some(cwd));
}

impl State {
    fn change_root(&mut self, path: &Path) -> PathBuf { 
        self.root_cwd.join(path.strip_prefix(ROOT).unwrap())
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
        self.root_cwd = "/home/laperlej/Projects".into();
        self.userspace_configuration = configuration;
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
        scan_host_folder(&Path::new(ROOT));
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
        self.dirlist.render();
        println!();
        println!();
        self.textinput.render(rows, cols);
    }
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
