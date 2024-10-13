use zellij_tile::prelude::*;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::path::Path;

use config::Config;

mod config;
mod filter;
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
    current_session: String,

    config: Config,
    debug: String,
}

register_plugin!(State);

impl State {
    fn change_root(&mut self, path: &Path) -> PathBuf { 
        self.cwd.join(path.strip_prefix(ROOT).unwrap())
    }

    fn switch_session_with_cwd(&self, dir: &Path) -> Result<(), String> {
        let session_name = dir.file_name().unwrap().to_str().unwrap();
        let cwd = dir.to_path_buf();
        let layout = self.config.layout.clone();
        // Switch session will panic if the session is the current session
        if session_name != self.current_session {
            switch_session_with_layout(Some(session_name), layout, Some(cwd));
        }
        Ok(())
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
        for dir in &self.config.dirs {
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
                let dirs = self.make_dirlist(&paths);
                self.dirlist.update_dirs(dirs);
                should_render = true;
            },
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
                    Key::Esc => {
                        close_self();
                    }
                    Key::Down => {
                        self.dirlist.handle_down();
                    }
                    Key::Up => {
                        self.dirlist.handle_up();
                    }
                    Key::Char('\n') | Key::Char('\r') => {
                        if let Some(selected) = self.dirlist.get_selected() {
                            let _ = self.switch_session_with_cwd(Path::new(&selected));
                            close_self();
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
        self.dirlist.render(rows.saturating_sub(4), cols);
        println!();
        self.textinput.render(rows, cols);
        println!();
        if !self.debug.is_empty() {
            println!();
            println!("{}", self.debug);
        }
    }
}

fn is_hidden(path: &Path) -> bool {
    const WHITELIST: [&str; 1] = [".config"];

    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.') && !WHITELIST.contains(&s))
        .unwrap_or(false)
}
