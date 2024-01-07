use std::{env, path::Path};

mod plugin_manager;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let plugin_path = env::args().nth(1).expect("no plugin path was given");
    let mut plugin_manager = plugin_manager::PluginManager::new();
    unsafe {
        plugin_manager
            .load_plugin(Path::new(&plugin_path))
            .expect(&format!("cloud not load plugin at {plugin_path}"));
    }
}
