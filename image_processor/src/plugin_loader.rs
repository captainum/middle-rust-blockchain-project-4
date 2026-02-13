use core::ffi::{c_char, c_void};
use libloading::{Library, Symbol};
use std::path::Path;

pub struct Plugin {
    plugin: Library,
}

pub struct PluginInterface<'a> {
    pub process_image: Symbol<
        'a,
        unsafe extern "C" fn(
            width: u32,
            height: u32,
            rgba_data: *mut u8,
            params: *const c_char,
        ) -> c_void,
    >,
}

impl Plugin {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Err(anyhow::anyhow!(
                "Библиотеки плагина {} не существует!",
                path.display()
            ));
        }

        let plugin = unsafe { Library::new(path) }?;

        Ok(Self { plugin })
    }

    pub fn interface(&self) -> Result<PluginInterface<'_>, libloading::Error> {
        Ok(PluginInterface {
            process_image: unsafe { self.plugin.get("process_image") }?,
        })
    }
}
