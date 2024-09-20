use zellij_tile::prelude::*;

type Color = usize;

#[derive(Debug)]
pub struct TextInput {
    text: Vec<char>,

    cursor_symbol: char,
    marker_symbol: char,
    marker_color: Color,
    text_color: Color,
    cursor_color: Color,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: Vec::new(),

            marker_symbol: '>',
            cursor_symbol: '_',
            text_color: 0,
            cursor_color: 1,
            marker_color: 2,
        }
    }
}

impl TextInput {
    pub fn reset(&mut self) {
        self.text.clear();
    }
    
    pub fn get_text(&self) -> String {
        self.text.iter().collect::<String>()
    }

    pub fn handle_backspace(&mut self) {
            self.text.pop();
    }

    pub fn handle_char(&mut self, c: char) {
        if c == '\n' || c == '\r' || c == '\t' {
            return;
        }
        self.text.push(c);
    }

    pub fn render(&self, _rows: usize, _cols: usize) {
        let search_term = self.text.iter().collect::<String>();
        let search_bar_content = format!("{} {}{}", self.marker_symbol, search_term, self.cursor_symbol);
        let search_bar_len = search_bar_content.len();
        let search_bar = Text::new(search_bar_content)
            .color_range(self.marker_color, 0..1)
            .color_range(self.text_color, 2..search_bar_len)
            .color_range(self.cursor_color, search_bar_len..search_bar_len + 1);
        print_text(search_bar);
    }
}
