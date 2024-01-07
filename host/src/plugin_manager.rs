use std::path::Path;

use libloading::{Library, Symbol};

use plugin_interface::Plugin;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libs: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            loaded_libs: Vec::new(),
        }
    }

    pub unsafe fn load_plugin(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib = Library::new(path)?;
        self.loaded_libs.push(lib);

        let lib = self.loaded_libs.last().unwrap();
        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;
        let boxed_raw = constructor();
        let plugin = Box::from_raw(boxed_raw);
        log::debug!("loaded plugin {:?}", plugin.id());
        plugin.on_plugin_load();
        self.plugins.push(plugin);

        Ok(())
    }

    pub fn unload_all_plugins(&mut self) {
        for plugin in self.plugins.drain(..) {
            log::trace!("invoking on_plugin_unload for {:?}", plugin.id());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libs.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libs.is_empty() {
            self.unload_all_plugins();
        }
    }
}
