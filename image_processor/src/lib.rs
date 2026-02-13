use image::DynamicImage;
use image::codecs::png::PngDecoder;
use std::io::BufReader;
use std::path::Path;

pub mod plugin_loader;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

pub fn load_png(path: &Path) -> anyhow::Result<Image> {
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Файл с исходным PNG-изображением {} не существует!",
            path.display()
        ));
    }

    let mut input = BufReader::new(std::fs::File::open(path)?);

    let decoder = PngDecoder::new(&mut input)?;

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
