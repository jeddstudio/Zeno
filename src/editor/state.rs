use std::ops::Range;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EditorState {
    text: String,
    anchor: usize,
    cursor: usize,
}

impl EditorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn selection_range(&self) -> Range<usize> {
        let start = self.anchor.min(self.cursor);
        let end = self.anchor.max(self.cursor);
        start..end
    }

    pub fn selection_reversed(&self) -> bool {
        self.cursor < self.anchor
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.cursor = self.cursor.min(self.text.len());
        self.anchor = self.anchor.min(self.text.len());
        self.collapse_selection();
    }

    pub fn collapse_selection(&mut self) {
        self.anchor = self.cursor;
    }

    pub fn insert_str(&mut self, s: &str) {
        let range = self.selection_range();
        self.replace_range(range, s);
    }

    pub fn backspace(&mut self) {
        let selection = self.selection_range();
        if !selection.is_empty() {
            self.replace_range(selection, "");
            return;
        }

        if self.cursor == 0 {
            return;
        }

        let prev = prev_char_boundary(&self.text, self.cursor);
        self.replace_range(prev..self.cursor, "");
    }

    pub fn delete_forward(&mut self) {
        let selection = self.selection_range();
        if !selection.is_empty() {
            self.replace_range(selection, "");
            return;
        }

        if self.cursor >= self.text.len() {
            return;
        }

        let next = next_char_boundary(&self.text, self.cursor);
        self.replace_range(self.cursor..next, "");
    }

    pub fn move_left(&mut self, extend_selection: bool) {
        if !extend_selection {
            let selection = self.selection_range();
            if !selection.is_empty() {
                self.cursor = selection.start;
                self.anchor = self.cursor;
                return;
            }
        }

        self.cursor = prev_char_boundary(&self.text, self.cursor);
        if !extend_selection {
            self.anchor = self.cursor;
        }
    }

    pub fn move_right(&mut self, extend_selection: bool) {
        if !extend_selection {
            let selection = self.selection_range();
            if !selection.is_empty() {
                self.cursor = selection.end;
                self.anchor = self.cursor;
                return;
            }
        }

        self.cursor = next_char_boundary(&self.text, self.cursor);
        if !extend_selection {
            self.anchor = self.cursor;
        }
    }

    pub fn select_all(&mut self) {
        self.anchor = 0;
        self.cursor = self.text.len();
    }

    pub fn replace_range(&mut self, range: Range<usize>, new_text: &str) {
        let range = normalize_range_to_char_boundaries(&self.text, range);
        if range.start > range.end || range.end > self.text.len() {
            return;
        }

        let mut next =
            String::with_capacity(self.text.len() - (range.end - range.start) + new_text.len());
        next.push_str(&self.text[..range.start]);
        next.push_str(new_text);
        next.push_str(&self.text[range.end..]);
        self.text = next;

        let next_cursor = range.start + new_text.len();
        self.cursor = next_cursor.min(self.text.len());
        self.anchor = self.cursor;
    }
}

fn prev_char_boundary(s: &str, idx: usize) -> usize {
    let idx = idx.min(s.len());
    if idx == 0 {
        return 0;
    }
    let mut i = idx - 1;
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

fn next_char_boundary(s: &str, idx: usize) -> usize {
    let idx = idx.min(s.len());
    if idx == s.len() {
        return idx;
    }
    let mut i = idx + 1;
    while i < s.len() && !s.is_char_boundary(i) {
        i += 1;
    }
    i.min(s.len())
}

fn normalize_range_to_char_boundaries(s: &str, range: Range<usize>) -> Range<usize> {
    let mut start = range.start.min(s.len());
    let mut end = range.end.min(s.len());
    if start > end {
        (start, end) = (end, start);
    }
    while start > 0 && !s.is_char_boundary(start) {
        start -= 1;
    }
    while end < s.len() && !s.is_char_boundary(end) {
        end += 1;
    }
    start..end.min(s.len())
}

#[cfg(test)]
mod tests {
    use super::EditorState;

    #[test]
    fn inserts_characters() {
        let mut editor = EditorState::new();
        editor.insert_str("a");
        editor.insert_str("b");
        editor.insert_str("c");
        assert_eq!(editor.text(), "abc");
        assert_eq!(editor.cursor(), 3);
        assert!(editor.selection_range().is_empty());
    }

    #[test]
    fn backspace_deletes_previous_character() {
        let mut editor = EditorState::new();
        editor.insert_str("ab");
        editor.backspace();
        assert_eq!(editor.text(), "a");
        assert_eq!(editor.cursor(), 1);
        editor.backspace();
        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), 0);
        editor.backspace();
        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor(), 0);
    }

    #[test]
    fn cursor_moves_left_and_right_with_bounds() {
        let mut editor = EditorState::new();
        editor.insert_str("ab");
        editor.move_left(false);
        assert_eq!(editor.cursor(), 1);
        editor.move_left(false);
        assert_eq!(editor.cursor(), 0);
        editor.move_left(false);
        assert_eq!(editor.cursor(), 0);
        editor.move_right(false);
        assert_eq!(editor.cursor(), 1);
        editor.move_right(false);
        assert_eq!(editor.cursor(), 2);
        editor.move_right(false);
        assert_eq!(editor.cursor(), 2);
    }

    #[test]
    fn newline_inserts_and_moves_cursor() {
        let mut editor = EditorState::new();
        editor.insert_str("a");
        editor.insert_str("\n");
        editor.insert_str("b");
        assert_eq!(editor.text(), "a\nb");
        assert_eq!(editor.cursor(), 3);
    }
}
