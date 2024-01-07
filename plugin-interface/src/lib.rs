use std::any::Any;

/// trait that defines the plugin interface
pub trait Plugin: Any + Send + Sync {
    /// get plugin identifier; must be unique
    fn id(&self) -> &'static str;
    // TODO: consider enforcing "one plugin per crate" by using cargo env variables
    // fn id(&self) -> String {
    //     let name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown");
    //     let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");
    //     format!("{name}@v{version}")
    // }

    /// a callback fires immediately after the plugin is loaded
    fn on_plugin_load(&self) {}

    /// a callback fires immediately before the plugin is unloaded
    fn on_plugin_unload(&self) {}

    // TODO: define the rest of the plugin interface (application specific)
}

/// declare a plugin type and its constructor
///
/// # Notes
///
/// This works by automatically generating a function with a pre-defined
/// signature and symbol name. Therefore you will only be able to declare
/// one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub fn _plugin_create() -> *mut dyn $crate::Plugin {
            // make sure the constructor is the correct type
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
