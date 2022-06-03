use std::env;

use libloading::Library;
use log::{debug, trace};

/// A plugin which allows you to add extra functionality.
///
/// example plugin:
///
/// ```
/// use plugins_loader::{Plugin, PluginRegistrar};
///
/// struct PluginTest;
///
///     impl Plugin for PluginTest {
///     fn name(&self) -> &'static str {
///         "plugin-name"
///     }
///
///     fn on_plugin_load(&self) {}
///
///     fn on_plugin_unload(&self) {}
///
///     fn execute(&self) {
///         println!("A command from `{}` plugin has been executed", self.name())
///     }
/// }
/// ```
pub trait Plugin {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A function that runs immediately after plugin loading.
    /// Usually used for initialization.
    fn on_plugin_load(&self);
    /// A function that runs immediately before the plugin is unloaded.
    /// Use this if you want to do any cleanup.
    fn on_plugin_unload(&self);
    /// The function will be executed, for example, when sending a message.
    fn execute(&self);
}

/// Plugin Manager
///
/// for example:
///
/// ```
/// use plugins_loader::loader;
///
/// let plugins = loader()?;
///
/// for plugin in &plugins.plugins {
///     plugin.execute()
/// }
/// ```
pub struct PluginManager {
    /// Vector with loaded plugins
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    /// Create empty `PluginManager`
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Drop loaded Plugins from memory
impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() {
            self.unload();
        }
    }
}

pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

impl PluginRegistrar for PluginManager {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin)
    }
}

pub fn loader() -> anyhow::Result<PluginManager> {
    // get path to .so lib from command argument
    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");

    // create a plugin manager where all loaded plugins will be located
    let mut plugin_manager = PluginManager::new();

    // loading library with .so is unsafe
    unsafe {
        // load library
        // Box::new and Box::leak must be there because if it isn't there it throws a segmentation fault
        let lib = Box::leak(Box::new(Library::new(library_path)?));

        // get `plugin_entry` from library
        let func: libloading::Symbol<unsafe extern "C" fn(&mut dyn PluginRegistrar) -> ()> =
            lib.get(b"plugin_entry")?;

        // execute initial function
        func(&mut plugin_manager);
    }

    Ok(plugin_manager)
}
