/// Tests that verify:
/// - All plugins have a manifest
/// - All plugins have the functions specified in the manifest
#[cfg(test)]
mod rs_plugins;

/// Tests the Fe Plugin
#[cfg(test)]
mod fe_plugin;

/// Tests the Framework Plugin
#[cfg(test)]
mod framework_plugin;

/// Tests the Manager Plugin
#[cfg(test)]
mod manager_plugin;
