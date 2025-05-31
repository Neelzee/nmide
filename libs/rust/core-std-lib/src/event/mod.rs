use crate::state::Value;
use dialog_builder::{DialogBuilder, DialogFileBuilder};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod dialog_builder;

#[cfg(feature = "core")]
pub mod core_events;

pub mod utils;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Serialize, Deserialize, TS, Hash, Eq)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum Event {
    Event {
        event: String,
        args: Option<Value>,
    },
    DialogEvent {
        event: String,
        kind: Option<DialogEvtKind>,
        message: String,
        btn: Option<DialogBtn>,
        title: Option<String>,
    },
    DialogFile {
        event: String,
        title: Option<String>,
        file_kind: DialogFileKind,
        filter_ext: Vec<String>,
        create_dirs: bool,
    },
    #[serde(rename = "nmide://post-init")]
    PostInit,
    #[serde(rename = "nmide://pre-exit")]
    PreExit,
    CoreResponse {
        event: String,
        args: Option<Value>,
    },
}

#[derive(
    Debug, Clone, PartialEq, PartialOrd, Ord, Serialize, Deserialize, TS, Hash, Eq, Default,
)]
#[serde(rename_all = "camelCase")]
pub enum DialogFileKind {
    #[default]
    SingleFile,
    SingleDir,
    MultiFile,
    MultiDir,
    SaveFile,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Serialize, Deserialize, TS, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DialogEvtKind {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Serialize, Deserialize, TS, Hash, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DialogBtn {
    Ok,
    OkCancel,
    YesNo,
    OkCustom(String),
    OkCancelCustom(String, String),
}

impl Event {
    pub fn new<S: ToString>(event_name: S, args: Option<Value>) -> Self {
        Self::Event {
            event: event_name.to_string(),
            args,
        }
    }

    pub fn new_dialog() -> DialogBuilder {
        DialogBuilder::default()
    }

    pub fn new_file_dialog() -> DialogFileBuilder {
        DialogFileBuilder::default()
    }

    pub fn event_name(&self) -> &str {
        match self {
            Event::Event { event, .. } => event,
            Event::DialogEvent { event, .. } => event,
            Event::DialogFile { event, .. } => event,
            Event::PostInit => "nmide://post-init",
            Event::PreExit => "nmide://pre-exit",
            Event::CoreResponse { event, .. } => event,
        }
    }

    pub fn args(&self) -> Option<&Value> {
        match self {
            Event::Event { args, .. } => args.as_ref(),
            Event::DialogEvent { .. } => None,
            Event::DialogFile { .. } => None,
            Event::PostInit => None,
            Event::PreExit => None,
            Event::CoreResponse { args, .. } => args.as_ref(),
        }
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.event_name())
    }
}
