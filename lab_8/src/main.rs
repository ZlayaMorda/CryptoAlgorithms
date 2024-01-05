// SPDX-License-Identifier: MPL-2.0

use std::env;
use fft2d::slice::{fft_2d, fftshift, ifft_2d, ifftshift};
use rusttype::{Font, Scale, point};
use rustfft::{num_complex::Complex};
use image::{GrayImage, ImageBuffer, Luma, Rgba};
use show_image::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];
    let img_str = &args[2];


    if mode == "store" {

        let message = &args[3];
        let font_size: f32 = args[4].parse()?;
        // Open image from disk.
        let img = image::open(format!("{img_str}.jpg"))?.into_luma8();
        let (width, height) = img.dimensions();
        let image_font = create_image(width, height, message, font_size)?;
        let image_font_vec = image_font.to_vec();


        let window_in = create_window("input image", Default::default())?;
        let image_in = ImageView::new(ImageInfo::mono8(width, height), &img);
        window_in.set_image("input image", image_in)?;

        // Convert the image buffer to complex numbers to be able to compute the FFT.
        let mut img_buffer: Vec<Complex<f64>> = img
            .as_raw()
            .iter()
            .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
            .collect();

        fft_2d(width as usize, height as usize, &mut img_buffer);
        // Shift opposite quadrants of the fft (like matlab fftshift).
        img_buffer = fftshift(height as usize, width as usize, &img_buffer);

        for i in 0..img_buffer.len() {
            img_buffer[i].im += image_font_vec[i] as f64;
        }

        let img_freq_list = view_fft_norm(width, height, &img_buffer);

        println!("{:?}", img_buffer.len());
        let image_freq = ImageView::new(ImageInfo::mono8(width, height), &img_freq_list);
        let window_freq = create_window("freq image", Default::default())?;
        window_freq.set_image("image-freq", image_freq)?;


        // Invert the FFT back to the spatial domain of the image.
        img_buffer = ifftshift(height as usize, width as usize, &img_buffer);
        ifft_2d(height as usize, width as usize, &mut img_buffer);
        let fft_coef = 1.0 / (width * height) as f64;
        for x in img_buffer.iter_mut() {
            *x *= fft_coef;
        }

        // Convert the complex img_buffer back into a gray image.
        let img_raw: Vec<u8> = img_buffer
            .iter()
            .map(|c| (c.norm() * 255.0) as u8)
            .collect();
        // print!("{}", img_raw.len());
        let out_img = //DynamicImage::ImageRgba8(ImageBuffer::from_vec(width, height, img_raw).ok_or("Failed to create image buffer")?);
        GrayImage::from_raw(width, height, img_raw.clone()).unwrap();
        let image_out = ImageView::new(ImageInfo::mono8(width, height), &out_img);
        let window_out = create_window("out image", Default::default())?;
        window_out.set_image("image-out", image_out)?;

        let mut img_buffer: Vec<Complex<f64>> = out_img
            .as_raw()
            .iter()
            .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
            .collect();

        fft_2d(width as usize, height as usize, &mut img_buffer);
        // Shift opposite quadrants of the fft (like matlab fftshift).
        img_buffer = fftshift(height as usize, width as usize, &img_buffer);
        let img_freq_list = view_fft_norm(width, height, &img_buffer);
        println!("{:?}", img_buffer.len());
        let image_freq_out = ImageView::new(ImageInfo::mono8(width, height), &img_freq_list);
        let window_freq_out = create_window("freq image out", Default::default())?;
        window_freq_out.set_image("image-freq-out", image_freq_out)?;

        window_in.wait_until_destroyed()?;
        out_img.save(format!("{img_str}_modified.jpg"))?;
    }
    else {
        let img = image::open(format!("{img_str}_modified.jpg"))?.into_luma8();
        let (width, height) = img.dimensions();
        // Convert the image buffer to complex numbers to be able to compute the FFT.
        let mut img_buffer: Vec<Complex<f64>> = img
            .as_raw()
            .iter()
            .map(|&pix| Complex::new(pix as f64 / 255.0, 0.0))
            .collect();

        fft_2d(width as usize, height as usize, &mut img_buffer);
        // Shift opposite quadrants of the fft (like matlab fftshift).
        img_buffer = fftshift(height as usize, width as usize, &img_buffer);
        let img_freq_list = view_fft_norm(width, height, &img_buffer);

        println!("{:?}", img_buffer.len());
        let image_freq = ImageView::new(ImageInfo::mono8(width, height), &img_freq_list);
        let window_freq = create_window("freq image", Default::default())?;
        window_freq.set_image("image-freq", image_freq)?;
        window_freq.wait_until_destroyed()?;
    }

    Ok(())
}

/// Convert the norm of the (transposed) FFT 2d transform into an image for visualization.
/// Use a logarithm scale.
fn view_fft_norm(width: u32, height: u32, img_buffer: &[Complex<f64>]) -> GrayImage {
    let fft_log_norm: Vec<f64> = img_buffer.iter().map(|c| c.norm().ln()).collect();
    let max_norm = fft_log_norm.iter().cloned().fold(0.0 / 0.0, f64::max);
    let fft_norm_u8: Vec<u8> = fft_log_norm
        .into_iter()
        .map(|x| ((x / max_norm) * 255.0) as u8)
        .collect();
    GrayImage::from_raw(width, height, fft_norm_u8).unwrap()
}


fn create_image(width: u32, height: u32, text: &str, scale: f32) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>>{
    let mut img = ImageBuffer::new(width, height);
    // Load a font
    let font_data = include_bytes!("../font.ttf"); // Replace with your font path
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // Define the font size
    let scale = Scale::uniform(scale); //24.0

    // Render the text onto the image
    draw_text(&mut img, &text, &font, scale, 0, 0, Luma([255_u8]));
    let image = GrayImage::from_raw(width, height, img.to_vec()).unwrap();
    let image_font = ImageView::new(ImageInfo::mono8(width, height), &image.as_raw());
    let window_font = create_window("font image", Default::default())?;
    window_font.set_image("image-font", image_font)?;
    //window_font.wait_until_destroyed()?;
    Ok(image)
}

// Function to render text onto the image
fn draw_text(
    image: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
    text: &str,
    font: &Font<'static>,
    scale: Scale,
    x: i32,
    y: i32,
    color: Luma<u8>,
) {
    let v_metrics = font.v_metrics(scale);
    let offset = point(x as f32, y as f32 + v_metrics.ascent);

    for glyph in font.layout(text, scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, intensity| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;

                let pixel = image.get_pixel_mut(gx as u32, gy as u32);
                *pixel = Luma([(color[0] as f32 * intensity) as u8])
            });
        }
    }
}