use image::GenericImageView;
use image::{ImageBuffer, Rgb};
use minifb::{Key, ScaleMode, Window, WindowOptions};
use rand::Rng;
use std::env;

fn main() {
    // Parse args and load the image
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let image = image::open(file_name).unwrap();
    let number_of_iterations: usize = args[2].parse::<usize>().unwrap();
    let (width, height) = image.dimensions();

    // Convert the image to RGBA8 format
    let image_data = image.to_rgba8().into_raw();

    // Convert the image data to u32 format
    let mut pixel_buffer = Vec::new();
    for pixel in image_data.chunks_exact(4) {
        let red = pixel[0] as u32;
        let green = pixel[1] as u32;
        let blue = pixel[2] as u32;
        let u32_pixel = (red << 16) | (green << 8) | blue;
        pixel_buffer.push(u32_pixel);
    }

    // Create a window with the same dimensions as the image
    let mut window = Window::new(
        "Input image",
        width as usize,
        height as usize,
        WindowOptions {
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    )
    .unwrap();

    // Display the image in the window
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&pixel_buffer, width as usize, height as usize)
            .unwrap();
    }

    let mut rng = rand::thread_rng();

    // Create new image
    let mut new_image_sampled = Vec::new();

    // Fill with black color
    for _pixel in image_data.chunks_exact(4) {
        let u32_pixel = 0;
        new_image_sampled.push(u32_pixel);
    }

    for _ in 0..number_of_iterations {
        let rand_pixel: usize = rng.gen_range(0..(width * height) as usize);
        new_image_sampled[rand_pixel] = pixel_buffer[rand_pixel];
    }

    let mut window_output = Window::new(
        "Output image",
        width as usize,
        height as usize,
        WindowOptions {
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        },
    )
    .unwrap();
    while window_output.is_open() && !window_output.is_key_down(Key::Escape) {
        window_output
            .update_with_buffer(&new_image_sampled, width as usize, height as usize)
            .unwrap();
    }

    let new_image_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            new_image_sampled.as_ptr() as *const u8,
            new_image_sampled.len() * std::mem::size_of::<u32>(),
        )
    };
    let file_name_output = format!("{}-{}-processed", file_name, number_of_iterations);

    image::save_buffer(
        format!("processed-{file_name_output}.jpeg"),
        &new_image_bytes,
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();

    image::save_buffer_with_format(
        format!("normal-blued-colors-{file_name_output}.jpeg"),
        &new_image_bytes,
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
        image::ImageFormat::Jpeg,
    )
    .unwrap();


    // let new_image_bytes: &[u8] = unsafe {
    //     std::slice::from_raw_parts(
    //         new_image_sampled.as_ptr() as *const u8,
    //         new_image_sampled.len() * std::mem::size_of::<u32>(),
    //     )
    // };

    // let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, Rgb(new_image_bytes));
    // img.save_with_format("try_save_method.jpeg", image::ImageFormat::Jpeg).expect("Failed to save image");
    // image::save_buffer_with_format(
    //     format!("EXPERIMENTS-{file_name_output}.jpeg"),
    //     &new_image_bytes,
    //     width as u32,
    //     height as u32,
    //     image::ColorType::Rgba8,
    //     image::ImageFormat::Jpeg,
    // )
    // .unwrap();
}
