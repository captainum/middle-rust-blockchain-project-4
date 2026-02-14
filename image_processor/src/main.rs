//! CLI-приложение, которое загружает изображение, применяет к нему указанный плагин обработки и
//! сохраняет результат.

#![deny(unreachable_pub)]

use clap::Parser;
use image_processor::plugin_loader::Plugin;
use image_processor::{load_png, plugin_name_to_filename, read_params, save_png};
use std::ffi::CString;

use std::path::PathBuf;

/// Взаимодействие с CLI-приложением.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Путь к исходному PNG-изображению
    #[arg(long, value_name = "/path/to/png/image")]
    input: PathBuf,

    /// Путь, по которому будет сохранено обработанное изображение
    #[arg(long, value_name = "/path/to/save/destination/of/png/image")]
    output: PathBuf,

    /// Имя плагина (динамической библиотеки) без расширения (например, invert)
    #[arg(long, default_value = "blur_plugin")]
    plugin: String,

    /// Путь к текстовому файлу с параметрами обработки
    #[arg(long, value_name = "/path/to/params/file")]
    params: PathBuf,

    /// Путь к директории, где находится плагин
    #[arg(
        long,
        value_name = "/path/to/plugin/dir",
        default_value = "target/debug"
    )]
    plugin_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut img = load_png(&args.input)?;

    let width = img.width;
    let height = img.height;
    let rgba_data = &mut img.pixels;

    let params = read_params(&args.params)?;

    let plugin_path = args.plugin_path.join(plugin_name_to_filename(&args.plugin));

    let plugin = Plugin::new(&plugin_path)?;
    let process_image = plugin.interface()?.process_image;

    unsafe {
        process_image(
            width,
            height,
            rgba_data.as_mut_ptr(),
            CString::new(params)?.as_ptr(),
        )
    };

    save_png(&img, &args.output)?;

    Ok(())
}
