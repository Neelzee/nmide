use std::{fs::File, io::Read, path::Path};

use eyre::Result;

#[derive(Debug)]
pub struct WSFile {
    path: Box<Path>,
    name: String,
    ext: String,
    is_opened: bool,
    content: Option<String>,
    file: Box<File>,
}

impl WSFile {
    pub fn new(path: &Path) -> Option<Self> {
        if let Some(p) = path.to_str() {
            let path = Path::new(p);
            let file = File::open(path).ok()?;
            Some(WSFile {
                path: path.into(),
                name: path
                    .file_name()
                    .and_then(|e| e.to_str().and_then(|c| Some(c.to_string())))
                    .unwrap_or(p.to_string()),
                ext: path
                    .extension()
                    .and_then(|e| e.to_str().and_then(|c| Some(c.to_string())))
                    .unwrap_or("".to_string()),
                is_opened: false,
                content: None,
                file: Box::new(file),
            })
        } else {
            None
        }
    }

    pub fn open(&mut self) -> Result<()> {
        let mut file = File::open(self.path.clone())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        self.content = Some(buf);

        Ok(())
    }

    pub fn close(&mut self) {
        self.content = None;
        self.is_opened = false;
    }
}
