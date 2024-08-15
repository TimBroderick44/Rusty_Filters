use wasm_bindgen::prelude::*;
use image::{ImageBuffer, RgbaImage, Rgba};
use image::imageops::{grayscale, blur, huerotate, invert};
use image::codecs::png::PngEncoder;
use image::ColorType;
use std::io::Cursor;

// Exposes the apply_filter function to JavaScript, i.e. the function can be called from JavaScript
#[wasm_bindgen]
// It takes in a byte array and a string as input and returns a byte array as output
// '&' => Immutable reference, i.e. I can look but can't touch. It's like read-only. 
// 'u8' is a byte, i.e. ranges from 0 to 255 (just like a pixel value)
// 'Vec<u8>' is a vector (i.e. a dynamic array) of bytes (i.e. a dynamic array of pixel values)

pub fn apply_filter(img_data: &[u8], filter_type: &str) -> Vec<u8> {
    // Load the image from memory
    // The image crate supports and automatically detects a range of image formats
    let img = image::load_from_memory(img_data).expect("Failed to load image");
    
    // RGBA8 format is a common format for image processing
    let img = img.to_rgba8();

    // 'match' is like 'switch' in JS
    let processed_img: RgbaImage = match filter_type {
        "grayscale" => {
            let gray_img = grayscale(&img);
            // Need to convert the grayscale image to RGBA format
            // i.e. we iterate over the grayscale image and set the R, G, B values to the same value
            // and set the alpha value to 255 (i.e. fully opaque)
            // We use a closure = an anonymous function that doesn't have a name 
            // Syntax => |input1, input2, ...| { code }
            ImageBuffer::from_fn(gray_img.width(), gray_img.height(), |x, y| {
                let luma = gray_img.get_pixel(x, y)[0];
                Rgba([luma, luma, luma, 255])
            })
        },
        // 5.0 is the amount of blur
        "blur" => blur(&img, 5.0),
        // 90 is the angle by which the hue is 'rotated'
        "huerotate" => huerotate(&img, 90),
        "invert" => {
            // Clone the image so that the original image is not modified
            // mut => mutable reference, i.e. I can look and touch
            let mut img_clone = img.clone();
            invert(&mut img_clone);
            img_clone
        },
        "sepia" => apply_sepia(&img),
        "pixelate" => {
            // Basically, downscale so that quality is lost and then upscale to original size
            // Nearest => doesn't blend or smooth the pixels. Instead it just picks the nearest pixel
            let resized_img = image::imageops::resize(&img, img.width() / 10, img.height() / 10, image::imageops::FilterType::Nearest);
            // Resize the resized image back to the original size
            image::imageops::resize(&resized_img, img.width(), img.height(), image::imageops::FilterType::Nearest)
        },
        "emboss" => apply_emboss(&img),
        "sharpen" => apply_sharpen(&img),
        // 4 is the number of levels (i.e. the number of colors in the image)
        "posterize" => apply_posterize(&img, 4),
        // '_' => If not recognized, return the original image
        _ => img,
    };

    // Encode the processed image as PNG
    let mut buffer = Vec::new();
    // Cursor is a type that allows you to write to a buffer as if it were a file
    let mut cursor = Cursor::new(&mut buffer);
    // PngEncoder is a type that allows you to encode an image as a PNG
    let encoder = PngEncoder::new(&mut cursor);
    encoder
        .encode(&processed_img, processed_img.width(), processed_img.height(), ColorType::Rgba8)
        .expect("Failed to encode image");

    // buffer is returned as a byte array
    buffer
}

// kernel is a small grid or matrix that is used in image processing to apply effects and filters
// for each filter a different kernel is created
// f32 is a 32-bit floating point number
// 3 x 3 matrix => middle pixel is the target pixel and the surrounding pixels are multiplied by the surrounding values

fn apply_emboss(img: &RgbaImage) -> RgbaImage {
    let kernel: [[f32; 3]; 3] = [
        [-2.0, -1.0, 0.0],
        [-1.0,  1.0, 1.0],
        [ 0.0,  1.0, 2.0],
    ];
    apply_convolution(img, &kernel)
}

fn apply_sharpen(img: &RgbaImage) -> RgbaImage {
    let kernel: [[f32; 3]; 3] = [
        [ 0.0, -1.0,  0.0],
        [-1.0,  5.0, -1.0],
        [ 0.0, -1.0,  0.0],
    ];
    apply_convolution(img, &kernel)
}

fn apply_convolution(img: &RgbaImage, kernel: &[[f32; 3]; 3]) -> RgbaImage {
    // Get the dimensions (width and height) of the input image
    let (width, height) = img.dimensions();
    
    // Create a new image (output buffer) with the same dimensions as the original image
    let mut output = RgbaImage::new(width, height);

    // Loop over each pixel in the image, except for the edge pixels
    // (Edge pixels obviously don't have enough neighbors to apply the 3x3 kernel)
    for y in 1..(height - 1) { // Start at 1 and end at height-1 to avoid edges
        for x in 1..(width - 1) { // Start at 1 and end at width-1 to avoid edges
            
            // Initialize channel values
            // These will store the sum of the products of the kernel and the surrounding pixel values
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;

            // Nested loop to go through each value in the 3x3 kernel
            for ky in 0..3 { // Loop over the kernel rows (0, 1, 2)
                for kx in 0..3 { // Loop over the kernel columns (0, 1, 2)
                    
                    // Get the pixel value from the original image at the corresponding position
                    // The position is offset by the current kernel position (kx and ky)
                    let px = img.get_pixel(x + kx as u32 - 1, y + ky as u32 - 1);
                    
                    // Multiply each channel (red, green, blue, alpha) of the pixel by the corresponding kernel value
                    // and add the result to the respective accumulator
                    sum_r += kernel[ky][kx] * px[0] as f32; // Red channel
                    sum_g += kernel[ky][kx] * px[1] as f32; // Green channel
                    sum_b += kernel[ky][kx] * px[2] as f32; // Blue channel
                    sum_a += kernel[ky][kx] * px[3] as f32; // Alpha channel
                }
            }

            // After processing all the surrounding pixels, clamp the resulting values
            // This ensures the values are within the valid range for image data (0 to 255)
            // Then cast the values to u8 (8-bit unsigned integers)
            output.put_pixel(x, y, Rgba([
                sum_r.clamp(0.0, 255.0) as u8, // Red channel
                sum_g.clamp(0.0, 255.0) as u8, // Green channel
                sum_b.clamp(0.0, 255.0) as u8, // Blue channel
                sum_a.clamp(0.0, 255.0) as u8, // Alpha channel
            ]));
        }
    }

    // Return the processed image stored in the output buffer
    output
}

fn apply_sepia(img: &RgbaImage) -> RgbaImage {
    // Create a mutable clone of the original image so that we can modify it
    let mut sepia_img = img.clone();
    
    // Iterate over each pixel in the cloned image
    for pixel in sepia_img.pixels_mut() {
        // Extract the red, green, and blue values from the current pixel
        let red = pixel[0] as f32;
        let green = pixel[1] as f32;
        let blue = pixel[2] as f32;

        // Apply the sepia transformation formula to each color channel
        let tr = (0.393 * red + 0.769 * green + 0.189 * blue).min(255.0) as u8; // New red value
        let tg = (0.349 * red + 0.686 * green + 0.168 * blue).min(255.0) as u8; // New green value
        let tb = (0.272 * red + 0.534 * green + 0.131 * blue).min(255.0) as u8; // New blue value

        // Set the pixel's red, green, and blue channels to the new sepia values
        pixel[0] = tr;
        pixel[1] = tg;
        pixel[2] = tb;
    }

    // Return the sepia-toned image
    sepia_img
}

fn apply_posterize(img: &RgbaImage, levels: u8) -> RgbaImage {
    // Create a mutable clone of the original image so that we can modify it
    let mut posterized_img = img.clone();
    
    // Calculate the step size based on the number of levels
    // This determines how much we reduce the color range
    let step = 255 / (levels - 1);
    
    // Iterate over each pixel in the cloned image
    for pixel in posterized_img.pixels_mut() {
        // Apply the 'posterization' by reducing the color resolution
        // The color is taken to the nearest multiple of the step size
        pixel[0] = (pixel[0] / step) * step; // Posterize red channel
        pixel[1] = (pixel[1] / step) * step; // Posterize green channel
        pixel[2] = (pixel[2] / step) * step; // Posterize blue channel
        // Alpha channel is left unchanged
    }

    // Return the posterized image
    posterized_img
}

