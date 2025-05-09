use crate::event::{DialogBtn, DialogEvtKind, DialogFileKind, Event};

#[derive(Debug, Default)]
pub struct DialogBuilder {
    event: Option<String>,
    kind: Option<DialogEvtKind>,
    message: Option<String>,
    btn: Option<DialogBtn>,
    title: Option<String>,
}

impl DialogBuilder {
    pub fn event<S: Into<String>>(self, event: S) -> Self {
        Self {
            event: Some(event.into()),
            ..self
        }
    }

    pub fn kind(self, kind: DialogEvtKind) -> Self {
        Self {
            kind: Some(kind),
            ..self
        }
    }

    pub fn msg<S: Into<String>>(self, txt: S) -> Self {
        Self {
            message: Some(txt.into()),
            ..self
        }
    }

    pub fn btn(self, btn: DialogBtn) -> Self {
        Self {
            btn: Some(btn),
            ..self
        }
    }

    pub fn title<S: Into<String>>(self, title: S) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn build(self) -> Result<Event, DialogBuilderError> {
        Ok(Event::DialogEvent {
            event: self.event.ok_or(DialogBuilderError::MissingEventName)?,
            kind: self.kind,
            message: self.message.ok_or(DialogBuilderError::MissingMessage)?,
            btn: self.btn,
            title: self.title,
        })
    }
}

pub enum DialogBuilderError {
    MissingEventName,
    MissingMessage,
}

#[derive(Debug, Default)]
pub struct DialogFileBuilder {
    event: Option<String>,
    title: Option<String>,
    file_kind: Option<DialogFileKind>,
    filter_ext: Vec<String>,
    create_dirs: Option<bool>,
}

impl DialogFileBuilder {
    pub fn event<S: Into<String>>(self, event: S) -> Self {
        Self {
            event: Some(event.into()),
            ..self
        }
    }

    pub fn title<S: Into<String>>(self, title: S) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    pub fn file_kind(self, kind: DialogFileKind) -> Self {
        Self {
            file_kind: Some(kind),
            ..self
        }
    }

    pub fn filter_ext<S: Into<String>>(self, xs: Vec<S>) -> Self {
        Self {
            filter_ext: xs.into_iter().map(|s| s.into()).collect(),
            ..self
        }
    }

    pub fn create_dirs(self, b: bool) -> Self {
        Self {
            create_dirs: Some(b),
            ..self
        }
    }

    pub fn build(self) -> Result<Event, DialogBuilderError> {
        Ok(Event::DialogFile {
            event: self.event.ok_or(DialogBuilderError::MissingEventName)?,
            title: self.title,
            file_kind: self.file_kind.unwrap_or_default(),
            filter_ext: self.filter_ext,
            create_dirs: self.create_dirs.unwrap_or(false),
        })
    }
}
