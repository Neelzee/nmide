use core_module_lib::Module;
use core_std_lib::core::Core as _;
use core_std_lib::{core_modification::CoreModification, html::Html, state::State};
use once_cell::sync;
use tokio::join;
use tokio::sync::mpsc;

use crate::{CONSUMER, Core, MODULE, MODULE_NAME, SENDER, STATE, THROWN_EVENTS, UI};

pub struct Suite;

static INITIALIZED: sync::OnceCell<()> = sync::OnceCell::new();

impl Suite {
    async fn first_init(&mut self) {
        if let Some(_) = INITIALIZED.get() {
            return;
        }
        INITIALIZED.set(()).unwrap();
        tokio::spawn({
            let (sender, mut recv) = mpsc::channel::<CoreModification>(100);
            SENDER.set(sender).expect("Sender is not set yet");
            async move {
                while let Some(mods) = recv.recv().await {
                    let state = Core.state().await;
                    let ui = Core.ui().await;

                    let (new_state, new_ui) = mods.build(state, ui);

                    let mut st = STATE.write().await;
                    let mut current_ui = UI.write().await;
                    *st = new_state;
                    *current_ui = new_ui;
                    let mut prov = THROWN_EVENTS.write().await;
                    let mut evts: Vec<_> = current_ui
                        .get_attrs()
                        .into_iter()
                        .filter_map(|a| a.get_event())
                        .collect();
                    prov.append(&mut evts);
                }
            }
        });
    }

    pub async fn initialize(&mut self, module: Box<dyn Module>) {
        self.first_init().await;
        let name = module.name().to_string();
        join!(
            async {
                *UI.write().await = Html::Main();
            },
            async {
                CONSUMER.write().await.clear();
            },
            async {
                THROWN_EVENTS.write().await.clear();
            },
            async {
                *MODULE_NAME.write().await = name;
            },
            async {
                *MODULE.write().await = module;
            },
            async {
                *STATE.write().await = State::default();
            }
        );
    }

    pub fn new() -> Self {
        Self
    }
}
