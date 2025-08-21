use core_std_lib::{core_modification::UIInstr, event::Event};

#[async_trait::async_trait]
pub trait App: Send + Sync {
    /// Emits a re-render notification
    async fn rerender(&self, instr: UIInstr);
    /// Emits an Event
    async fn event(&self, event: Event);
    /// Exits the application
    async fn exit(&self);
}

impl std::fmt::Debug for dyn App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("App")
    }
}
