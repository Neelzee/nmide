use core_std_lib::state::Value;

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

    pub fn from_value(value: Value) -> Option<Self> {
        match value {
            Value::Obj(mp) => {
                let map = mp.to_hm();
                let start_row_pos = map.get("start_row_pos").cloned().and_then(|a| a.int())?;
                let end_row_pos = map.get("end_row_pos").cloned().and_then(|a| a.int())?;
                let start_col_pos = map.get("start_col_pos").cloned().and_then(|a| a.int())?;
                let end_col_pos = map.get("end_col_pos").cloned().and_then(|a| a.int())?;

                let start_row_pos = start_row_pos.unsigned_abs() as usize;
                let end_row_pos = end_row_pos.unsigned_abs() as usize;
                let start_col_pos = start_col_pos.unsigned_abs() as usize;
                let end_col_pos = end_col_pos.unsigned_abs() as usize;

                Some(Self {
                    start_row_pos,
                    end_row_pos,
                    start_col_pos,
                    end_col_pos,
                })
            }
            _ => None,
        }
    }

    pub fn to_value(self) -> Value {
        Value::new_obj()
            .add("start_row_pos", Value::Int(self.start_row_pos as i32))
            .add("end_row_pos", Value::Int(self.end_row_pos as i32))
            .add("start_col_pos", Value::Int(self.start_col_pos as i32))
            .add("end_col_pos", Value::Int(self.end_col_pos as i32))
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

    pub fn from_value(value: Value) -> Option<Self> {
        match value {
            Value::Obj(mp) => {
                let map = mp.to_hm();
                let pos = Position::from_value(map.get("pos").cloned()?)?;

                if let Some(s) = map.get("char").and_then(|v| v.str()) {
                    return Some(Self::Token {
                        pos,
                        char: s.chars().find(|_| true)?,
                    });
                }

                if let Some(s) = map.get("line").and_then(|v| v.list()) {
                    return Some(Self::Line {
                        pos,
                        line: s
                            .into_iter()
                            .map(Self::from_value)
                            .collect::<Option<Vec<_>>>()?,
                    });
                }

                None
            }
            _ => None,
        }
    }

    pub fn to_value(self) -> Value {
        match self {
            Art::Token { pos, char } => Value::new_obj()
                .add("pos", pos.to_value())
                .add("char", Value::Str(char.to_string())),
            Art::Line { pos, line } => Value::new_obj().add("pos", pos.to_value()).add(
                "line",
                Value::List(line.into_iter().map(Self::to_value).collect()),
            ),
        }
    }
}
