//! Event Register
//!
//! Holds a mapping between modules and the Events they have registered for.
//! In the following diagram, we can see how the Event system work.
//!
//! A. Some Event is thrown from the Backend to the Frontend
//! B. Some Event might be thrown from the Frontend aswell, but it ends up in
//!    the Frontend first.
//! C. Finally, the Event is sent to the Backend
//!
//!```text
//!         ┌──┐
//!         ▼B │
//!    ┌──────────┐A
//!  ┌─│ Frontend │◄┐
//!  │ └──────────┘ │
//!  │C┌─────────┐  │
//!  └►│ Backend │──┘
//!    └─────────┘
//!```
//!
//! After the Event is sent to the Backend, `ModuleEventRegister` is invoked
//! with the Event name, getting all the modules who have registered for this
//! Event. Modules register for Events through the `Core`, but ends up here.
//! There is nothing stopping Modules from register another Module to an Event.
//!
//! If the string `"*"` is used, then the Module will be invoked on every Event.

use core_std_lib::event::Event;
use log::info;
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Holds a mapping between modules and the Events they have registered for.
#[derive(Default)]
pub struct ModuleEventRegister {
    /// Event name → List of Module names
    event: RwLock<HashMap<String, Vec<String>>>,
}

impl ModuleEventRegister {
    /// Retrievs all the modules that has registered for this Event. This
    /// includes modules who has registered for the `*` event.
    pub async fn get_module_names(&self, event: &Event) -> Vec<String> {
        let mut modules = Vec::new();

        modules.append(
            &mut self
                .event
                .read()
                .await
                .get(event.event_name())
                .cloned()
                .unwrap_or(Vec::new()),
        );

        modules.append(
            &mut self
                .event
                .read()
                .await
                .get("*")
                .cloned()
                .unwrap_or(Vec::new()),
        );

        modules
    }

    pub async fn register_module(&mut self, event: String, handler: String) {
        info!(
            place = "backend";
            "register module: {}, to event {:?}",
            handler, event
        );
        let mut modules = self.event.write().await;
        let mut vec = modules.get(&event).cloned().unwrap_or(Vec::new());
        vec.push(handler.clone());
        modules.insert(event, vec);
    }
}
