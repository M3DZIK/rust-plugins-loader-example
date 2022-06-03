use plugins_loader::{Plugin, PluginRegistrar};

struct PluginTest;

impl Plugin for PluginTest {
    fn name(&self) -> &'static str {
        "test"
    }

    fn on_plugin_load(&self) {}

    fn on_plugin_unload(&self) {}

    fn execute(&self) {
        println!("A command from `{}` plugin has been executed", self.name())
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn PluginRegistrar) {
    registrar.register_plugin(Box::new(PluginTest))
}
