mod test_caching {
    use crate::ModuleBuilder;
    use core_module_lib::{Module, ModuleBuilder as _};
    use debug_core::core::run::{run, CoreOptions};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test() {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        modules.insert("ide_fsa".to_string(), Box::new(ModuleBuilder.build()));
        let res = run(modules, CoreOptions::default()).await;
        println!("{res:?}");
    }
}
