//! CLI-приложение, которое загружает изображение, применяет к нему указанный плагин обработки и
//! сохраняет результат.

#![deny(unreachable_pub)]

use clap::Parser;
use image_processor::load_png;
use image_processor::plugin_loader::Plugin;
use std::io::BufRead;

use std::path::{Path, PathBuf};

/// Взаимодействие с CLI-приложением.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Путь к исходному PNG-изображению
    #[arg(long)]
    input: PathBuf,

    /// Путь, по которому будет сохранено обработанное изображение
    #[arg(long)]
    output: PathBuf,

    /// Имя плагина (динамической библиотеки) без расширения (например, invert)
    #[arg(long)]
    plugin: String,

    /// Путь к текстовому файлу с параметрами обработки
    #[arg(long)]
    params: PathBuf,

    /// Путь к директории, где находится плагин
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn plugin_name_to_filename(plugin: &str) -> String {
    if cfg!(target_os = "linux") {
        format!("lib{}.so", plugin)
    } else if cfg!(target_os = "macos") {
        format!("{}.dylib", plugin)
    } else if cfg!(target_os = "windows") {
        format!("{}.dll", plugin)
    } else {
        unimplemented!("unsupported platform")
    }
}

fn read_params(params: &Path) -> anyhow::Result<String> {
    if !params.exists() {
        return Err(anyhow::anyhow!(
            "Файл с параметрами обработки {} не существует!",
            params.display()
        ));
    }

    let f = std::fs::File::open(params)?;

    let mut result = String::new();
    std::io::BufReader::new(f).read_line(&mut result)?;

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let img = load_png(&args.input)?;

    let width = img.width;
    let height = img.height;
    let mut rgba_data = img.pixels;

    let params = read_params(&args.params)?;

    let plugin_path = args.plugin_path.join(plugin_name_to_filename(&args.plugin));

    let plugin = Plugin::new(&plugin_path)?;
    let process_image = plugin.interface()?.process_image;

    unsafe {
        process_image(
            width,
            height,
            rgba_data.as_mut_ptr(),
            params
                .as_bytes()
                .iter()
                .map(|&b| b as i8)
                .collect::<Vec<_>>()
                .as_ptr(),
        )
    };

    Ok(())
}
