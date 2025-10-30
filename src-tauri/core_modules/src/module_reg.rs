use core_module_lib::Module;
use core_module_lib::ModuleBuilder;
use std::collections::HashMap;
use ide_cache;
use ide_explorer;
use ide_framework;
use ide_fsa;
use ide_pm;
use ide_tabs;
use magnolia_dependency;
use module_installer;
use rust_dependency;
use tab_wrapper;
use trivial_module;
pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {
    modules.insert("ide_cache".to_string(), Box::new(ide_cache::ModuleBuilder.build()));
    modules.insert("ide_explorer".to_string(), Box::new(ide_explorer::ModuleBuilder.build()));
    modules.insert("ide_framework".to_string(), Box::new(ide_framework::ModuleBuilder.build()));
    modules.insert("ide_fsa".to_string(), Box::new(ide_fsa::ModuleBuilder.build()));
    modules.insert("ide_pm".to_string(), Box::new(ide_pm::ModuleBuilder.build()));
    modules.insert("ide_tabs".to_string(), Box::new(ide_tabs::ModuleBuilder.build()));
    modules.insert("magnolia_dependency".to_string(), Box::new(magnolia_dependency::ModuleBuilder.build()));
    modules.insert("module_installer".to_string(), Box::new(module_installer::ModuleBuilder.build()));
    modules.insert("rust_dependency".to_string(), Box::new(rust_dependency::ModuleBuilder.build()));
    modules.insert("tab_wrapper".to_string(), Box::new(tab_wrapper::ModuleBuilder.build()));
    modules.insert("trivial_module".to_string(), Box::new(trivial_module::ModuleBuilder.build()));
}
