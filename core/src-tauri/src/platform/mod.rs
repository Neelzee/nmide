use core_std_lib::{core_modification::UIInstr, event::Event};

pub mod server;
/// Contains Tauri specific code
pub mod tauri;

#[async_trait::async_trait]
pub trait Platform: Send + Sync + std::fmt::Debug {
    /// Emits a re-render notification
    async fn rerender(&self, instr: UIInstr);
    /// Emits an Event
    async fn event(&self, event: Event);
    /// Exits the application
    async fn exit(&self);
}
