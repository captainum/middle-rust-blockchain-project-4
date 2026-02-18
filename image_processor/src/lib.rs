//! Вспомогательные инструменты для преобразования изображений с помощью плагинов.

#![deny(unreachable_pub)]

use image::DynamicImage;
use image::codecs::png::PngDecoder;
use log::info;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub mod plugin_loader;

/// Информация об изображении.
pub struct Image {
    /// Ширина изображения (в пикселях).
    pub width: u32,

    /// Высота изображения (в пикселях).
    pub height: u32,

    /// Массив байтов размером width * height * 4 (RGBA).
    pub pixels: Vec<u8>,
}

/// Загрузить PNG-изображения по заданному пути.
pub fn load_png(path: &Path) -> anyhow::Result<Image> {
    info!("Загрузка PNG-изображения из {}", path.display());

    if !path.exists() {
        anyhow::bail!(
            "Файл с исходным PNG-изображением {} не существует!",
            path.display()
        );
    }

    let mut input = BufReader::new(std::fs::File::open(path)?);

    let decoder = PngDecoder::new(&mut input).map_err(|e| {
        anyhow::anyhow!("Произошла ошибка при попытке загрузки PNG-изображения: {e}")
    })?;

    let img = DynamicImage::from_decoder(decoder)?.to_rgba8();

    let width = img.width();
    let height = img.height();
    let pixels = img.pixels().flat_map(|pixel| pixel.0).collect::<Vec<_>>();

    Ok(Image {
        width,
        height,
        pixels,
    })
}

/// Сохранить PNG-изображения по заданному пути.
pub fn save_png(image: &Image, path: &Path) -> anyhow::Result<()> {
    info!("Сохранение PNG-изображения в {}", path.display());

    let path = match path.extension().and_then(|ext| ext.to_str()) {
        Some("png") => path.to_path_buf(),
        _ => path.with_extension("png"),
    };

    image::save_buffer(
        &path,
        &image.pixels,
        image.width,
        image.height,
        image::ColorType::Rgba8,
    )?;

    Ok(())
}

/// Преобразовать имя плагина, добавив ему нужные суффиксы (напр. lib) и постфиксы (напр. .so)
/// при необходимости.
pub fn plugin_name_to_filename(plugin: &str) -> String {
    if cfg!(target_os = "linux") {
        format!("lib{}.so", plugin)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", plugin)
    } else if cfg!(target_os = "windows") {
        format!("{}.dll", plugin)
    } else {
        unimplemented!("unsupported platform")
    }
}

/// Прочитать параметры из заданного файла.
pub fn read_params(params: &Path) -> anyhow::Result<String> {
    info!("Чтение параметров из {}", params.display());

    if !params.exists() {
        anyhow::bail!(
            "Файл с параметрами обработки {} не существует!",
            params.display()
        );
    }

    let f = std::fs::File::open(params)?;

    let mut result = String::new();
    BufReader::new(f).read_line(&mut result)?;

    Ok(result)
}
