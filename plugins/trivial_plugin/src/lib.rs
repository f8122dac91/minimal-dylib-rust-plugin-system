use plugin_interface::{declare_plugin, Plugin};

declare_plugin!(TrivialPlugin, TrivialPlugin::default);

const PLUGIN_ID: &str = "trivial-plugin";

#[derive(Debug, Default)]
pub struct TrivialPlugin;

impl Plugin for TrivialPlugin {
    fn id(&self) -> &'static str {
        PLUGIN_ID
    }

    fn on_plugin_load(&self) {
        println!("{} loaded", self.id());
    }

    fn on_plugin_unload(&self) {
        println!("{} unloaded", self.id());
    }
}
