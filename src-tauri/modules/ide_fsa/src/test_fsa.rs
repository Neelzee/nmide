mod test_caching {
    use crate::{ModuleBuilder, FSA_DIR, FSA_READ, FSA_WRITE};
    use core_module_lib::{Module, ModuleBuilder as _};
    use debug_core::core::run::{run, CoreOptions};
    use std::collections::HashMap;
    use core_std_lib::event::Event;
    use core_std_lib::event::Event::PreExit;

    #[tokio::test]
    async fn fsa_handles_none_args() {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        modules.insert("ide_fsa".to_string(), Box::new(ModuleBuilder.build()));
        let res = run(
            modules,
            CoreOptions::default()
                .post_init_events(vec![
                    Event::new(FSA_READ, None),
                    Event::new(FSA_WRITE, None),
                    Event::new(FSA_DIR, None),
                    PreExit
                ])
        ).await;

        assert_eq!(
            res.events_thrown.into_iter().filter(|e| e.event_name() == "fsa-error").count(),
            3,
            "fsa should throw 3 fsa error events"
        );
    }
}
