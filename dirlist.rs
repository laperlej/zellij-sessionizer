use zellij_tile::prelude::*;

use std::collections::HashSet;

use crate::filter;

#[derive(Debug, Default)]
pub struct DirList {
    unique: HashSet<String>,
    dirs: Vec<String>,
    sessions: Vec<String>,
    cursor: usize,

    search_term: String,
    filtered_dirs: Vec<String>,
}

impl DirList {
    pub fn reset(&mut self) {
        self.dirs.clear();
        self.cursor = 0;
        self.filtered_dirs.clear();
    }

    pub fn update_dirs(&mut self, dirs: Vec<String>) {
        dirs.iter().for_each(|dir| {
            if !self.unique.contains(dir) {
                self.unique.insert(dir.clone());
                self.dirs.push(dir.clone());
            }
        });
        self.dirs.sort_by(|a, b| b.cmp(a));
        self.cursor = self.dirs.len().saturating_sub(1);
        self.filter();
    }

    /// Final path component of every listed dir, to match against session names.
    pub fn dir_names(&self) -> HashSet<String> {
        self.dirs
            .iter()
            .filter_map(|d| d.rsplit(['/', '\\']).next())
            .map(String::from)
            .collect()
    }

    /// Replaces the active-session entries (ended sessions must disappear,
    /// unlike dirs which only ever accumulate).
    pub fn set_sessions(&mut self, sessions: Vec<String>) {
        self.sessions = sessions;
        self.filter();
    }

    pub fn handle_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn handle_down(&mut self) {
        if self.cursor < self.filtered_dirs.len().saturating_sub(1) {
            self.cursor += 1;
        }
    }

    pub fn handle_half_page_up(&mut self, rows: usize) {
        let half_page = rows / 2;
        self.cursor = self.cursor.saturating_sub(half_page);
    }

    pub fn handle_half_page_down(&mut self, rows: usize) {
        let half_page = rows / 2;
        self.cursor = self
            .cursor
            .saturating_add(half_page)
            .min(self.filtered_dirs.len().saturating_sub(1));
    }

    pub fn get_selected(&self) -> Option<String> {
        if self.cursor < self.filtered_dirs.len() {
            Some(self.filtered_dirs[self.cursor].clone())
        } else {
            None
        }
    }

    pub fn set_search_term(&mut self, search_term: &str) {
        self.search_term = search_term.to_string();
        self.filter();
    }

    pub fn filter(&mut self) {
        // A session named after a listed folder is already reachable through
        // that folder, so only show the others. Done here (not on session
        // update) because dirs can arrive after sessions.
        let dir_names = self.dir_names();
        let mut all: Vec<String> = self
            .dirs
            .iter()
            .chain(self.sessions.iter().filter(|s| !dir_names.contains(*s)))
            .cloned()
            .collect();
        all.sort_by(|a, b| b.cmp(a));
        self.filtered_dirs = filter::fuzzy_filter(&all, self.search_term.as_str());
        self.cursor = self.filtered_dirs.len().saturating_sub(1);
    }

    pub fn render(&self, rows: usize, _cols: usize) {
        let from = self
            .cursor
            .saturating_sub(rows.saturating_sub(1) / 2)
            .min(self.filtered_dirs.len().saturating_sub(rows));
        let missing_rows = rows.saturating_sub(self.filtered_dirs.len());
        if missing_rows > 0 {
            for _ in 0..missing_rows {
                println!();
            }
        }
        self.filtered_dirs
            .iter()
            .enumerate()
            .skip(from)
            .take(rows)
            .for_each(|(i, dir)| {
                let text = dir.to_string();
                let text_len = text.len();
                let item = Text::new(text);
                let item = match i == self.cursor {
                    true => item.color_range(0, 0..text_len).selected(),
                    false => item,
                };
                print_text(item);
                println!();
            })
    }
}
