pub enum Art {
    Token { pos: Position, char: char },
    Line { pos: Position, line: Vec<Art> },
}

pub struct Position {
    start_row_pos: usize,
    end_row_pos: usize,
    start_col_pos: usize,
    end_col_pos: usize,
}

impl Position {
    pub fn new_char_pos(line_index: usize, char_index: usize) -> Self {
        Self {
            start_row_pos: line_index,
            end_row_pos: line_index + 1,
            start_col_pos: char_index,
            end_col_pos: char_index + 1,
        }
    }

    pub fn new_line_pos(line_index: usize, line_len: usize) -> Self {
        Self {
            start_row_pos: line_index,
            end_row_pos: line_index + 1,
            start_col_pos: 0,
            end_col_pos: line_len,
        }
    }

    pub fn dec(&self) -> (usize, usize, usize, usize) {
        (
            self.start_row_pos,
            self.end_row_pos,
            self.start_col_pos,
            self.end_col_pos,
        )
    }
}

impl Art {
    pub fn parse(s: &str) -> Vec<Self> {
        s.lines()
            .enumerate()
            .map(|(line_index, line_str)| {
                let line = line_str
                    .chars()
                    .enumerate()
                    .map(|(i, c)| Art::Token {
                        pos: Position::new_char_pos(line_index, i),
                        char: c,
                    })
                    .collect::<Vec<_>>();

                Art::Line {
                    pos: Position::new_line_pos(line_index, line.len()),
                    line,
                }
            })
            .collect()
    }

    pub fn get_pos(&self) -> (usize, usize, usize, usize) {
        match self {
            Art::Token { pos, .. } | Art::Line { pos, .. } => pos.dec(),
        }
    }
}
