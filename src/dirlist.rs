use zellij_tile::prelude::*;

#[derive(Debug, Default)]
pub struct DirList {
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
        self.dirs = dirs;
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
        if self.cursor < self.dirs.len().saturating_sub(1) {
            self.cursor += 1;
        }
    }

    pub fn get_selected(&self) -> Option<String> {
        if self.cursor < self.dirs.len() {
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
        self.filtered_dirs = self.dirs
            .iter()
            .filter(|dir| dir.contains(self.search_term.as_str()))
            .map(|dir| dir.to_string())
            .collect();
        if self.cursor >= self.filtered_dirs.len() {
            self.cursor = self.filtered_dirs.len().saturating_sub(1);
        }
    }


    pub fn render(&self, rows: usize, _cols: usize) {
        let from = self.cursor.saturating_sub(rows.saturating_sub(1) / 2).min(self.filtered_dirs.len().saturating_sub(rows));
        let missing_rows = rows.saturating_sub(self.filtered_dirs.len());
        if missing_rows > 0 {
            for _ in 0..missing_rows {
                println!();
            }
        }
        let list_items = self.filtered_dirs
            .iter()
            .enumerate()
            .skip(from)
            .take(rows)
            .map(|(i, dir)| {
                let item = NestedListItem::new(dir.to_string());
                if i == self.cursor {
                    item.selected()
                } else {
                    item
                }
            })
            .collect();
        print_nested_list(list_items);
    }
}

