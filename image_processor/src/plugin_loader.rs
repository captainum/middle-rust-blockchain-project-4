//! Загрузчик плагина.

use core::ffi::{c_char, c_void};
use libloading::{Library, Symbol};
use std::path::Path;

/// Обертка над подключаемым плагином.
pub struct Plugin {
    /// Загружаемая динамическая библиотека.
    plugin: Library,
}

/// Интерфейс плагина.
pub struct PluginInterface<'a> {
    /// Функция обработки изображения.
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
    /// Загрузить плагин по заданному пути.
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

    /// Получить интерфейс плагина.
    pub fn interface(&self) -> Result<PluginInterface<'_>, libloading::Error> {
        Ok(PluginInterface {
            process_image: unsafe { self.plugin.get("process_image") }?,
        })
    }
}
