//! CLI-приложение, которое загружает изображение, применяет к нему указанный плагин обработки и
//! сохраняет результат.

#![deny(unreachable_pub)]

use clap::Parser;
use image_processor::plugin_loader::Plugin;
use image_processor::{load_png, plugin_name_to_filename, read_params, save_png};
use log::{debug, info};
use std::ffi::CString;
use std::path::PathBuf;

/// Взаимодействие с CLI-приложением.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Путь к исходному PNG-изображению (расширение - .png)
    #[arg(long, value_name = "/path/to/png/image.png")]
    input: PathBuf,

    /// Путь, по которому будет сохранено обработанное изображение (расширение - .png)
    #[arg(long, value_name = "/path/to/save/destination/of/png/image.png")]
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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Cli::parse();
    debug!("Аргументы CLI: {:?}", args);

    if args.output.extension().and_then(|e| e.to_str()) != Some("png") {
        anyhow::bail!(
            "Выходной файл должен иметь расширение .png, получено: {}",
            args.output.display()
        );
    }

    let mut img = load_png(&args.input)?;

    let width = img.width;
    let height = img.height;
    let rgba_data = &mut img.pixels;
    debug!(
        "Изображение загружено: {}x{}, {} байт",
        width,
        height,
        rgba_data.len()
    );

    let params = read_params(&args.params)?;
    debug!("Параметры: {:?}", params);

    let plugin_path = args.plugin_path.join(plugin_name_to_filename(&args.plugin));

    let plugin = Plugin::new(&plugin_path)?;
    let process_image = plugin.interface()?.process_image;

    info!("Применение плагина \"{}\"", args.plugin);
    unsafe {
        process_image(
            width,
            height,
            rgba_data.as_mut_ptr(),
            CString::new(params)?.as_ptr(),
        )
    };

    save_png(&img, &args.output)?;

    info!("Готово!");
    Ok(())
}
