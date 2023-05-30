use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel};
use rand::Rng;
use std::env;

struct ImageProcessor {
    file_name: String,
    percent_of_iterations: usize,
    img: DynamicImage,
    modified_image: DynamicImage,
}

enum Shape {
    Full,
    WidthSide,
    HeightSide, 
}

impl ImageProcessor {
    fn new(file_name: String, percent_of_iterations: usize) -> Self {
        let img = image::open(&file_name).expect("Failed to open image file");
        let modified_image = DynamicImage::new_rgba8(img.width(), img.height());

        ImageProcessor {
            file_name,
            percent_of_iterations,
            img,
            modified_image,
        }
    }

    fn calculate_number_of_iterations(&self, shape: Shape) -> usize {
        let (w, h) = (self.img.width(), self.img.height());
        let param: usize;
        match shape {
            Shape::Full => param = (w * h) as usize,
            Shape::WidthSide => param = w as usize,
            Shape::HeightSide => param = h as usize,
        };
        param / 100 * self.percent_of_iterations
    }

    

    fn random_sampling_dots(&mut self) {
        let number_of_iterations = self.calculate_number_of_iterations(Shape::Full);
        let mut rng = rand::thread_rng();

        for _ in 0..number_of_iterations {
            let random_x = rng.gen_range(0..self.img.width());
            let random_y = rng.gen_range(0..self.img.height());
            let random_pixel = self.img.get_pixel(random_x, random_y);
            self.modified_image
                .put_pixel(random_x, random_y, random_pixel);
        }
    }

    fn random_sampling_lines_x(&mut self) {
        let number_of_iterations = self.calculate_number_of_iterations(Shape::WidthSide);
        let mut rng = rand::thread_rng();

        for _ in 0..number_of_iterations {
            let random_x = rng.gen_range(0..self.img.width());
            for i in 0..self.img.height() {
                let random_pixel = self.img.get_pixel(random_x, i);
                self.modified_image
                .put_pixel(random_x, i, random_pixel);
            }
        }
    }

    fn random_sampling_lines_y(&mut self) {
        let number_of_iterations = self.calculate_number_of_iterations(Shape::HeightSide);
        let mut rng = rand::thread_rng();

        for _ in 0..number_of_iterations {
            let random_y = rng.gen_range(0..self.img.height());
            for i in 0..self.img.width() {
                let random_pixel = self.img.get_pixel(i, random_y);
                self.modified_image
                .put_pixel(i, random_y, random_pixel);
            }
        }
    }

    fn save_modified_image(&self, prefix: String) {
        let file_name_output = format!("{}-{}", prefix, self.file_name);
        self.modified_image
            .save(file_name_output)
            .expect("Failed to save modified image");
    }
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("No file name provided").clone();
    let percent_of_iterations = args
        .get(2)
        .and_then(|arg| arg.parse::<usize>().ok())
        .expect("Invalid percentage of iterations");

    // Process the image
    let mut image_sampled_dots = ImageProcessor::new(file_name.clone(), percent_of_iterations);
    image_sampled_dots.random_sampling_dots();
    image_sampled_dots.save_modified_image("Random-Sampled-Dots".to_string());

    let mut image_sampled_lines = ImageProcessor::new(file_name.clone(), percent_of_iterations);
    image_sampled_lines.random_sampling_lines_x();
    image_sampled_lines.random_sampling_lines_y();
    image_sampled_lines.save_modified_image("Random-Sampled-Lines".to_string());
}
