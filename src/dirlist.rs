use zellij_tile::prelude::*;

use std::collections::HashSet;

use crate::filter;

#[derive(Debug, Default)]
pub struct DirList {
    unique: HashSet<String>,
    dirs: Vec<String>,
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
        self.filtered_dirs = filter::fuzzy_filter(&self.dirs, self.search_term.as_str());
        self.cursor = self.filtered_dirs.len().saturating_sub(1);
    }


    pub fn render(&self, rows: usize, _cols: usize) {
        let from = self.cursor.saturating_sub(rows.saturating_sub(1) / 2).min(self.filtered_dirs.len().saturating_sub(rows));
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

