use image::DynamicImage;

use bevy::math::UVec2;

use image::ImageBuffer;

use bevy::asset::Handle;

use bevy::render::texture::Image;

use bevy::asset::Assets;

use bevy::prelude::ResMut;

use bevy::math::Vec2;

fn is_in_rhombus(point: Vec2, size: Vec2, mid: Vec2) -> bool {
    let point = point - mid;
    let size = size - mid;

    let point = point;
    let size = size;

    let point = (point / size).abs();

    point.x + point.y <= 1.0
}

/// This function builds your image, you can use any pixel format you like
fn make_rhombus_tile(
    canvas_size: UVec2,
    color: image::Rgba<u8>,
    border_width: u32,
    border_color: image::Rgba<u8>,
) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    println!("canvas_size: {:?}", canvas_size);
    let mut image = ImageBuffer::new(canvas_size.x, canvas_size.y);

    let f_canvas_size = canvas_size.as_vec2();
    let mid = f_canvas_size / 2.0;
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if is_in_rhombus(Vec2::new(x as f32, y as f32), f_canvas_size, mid) {
            *pixel = color;
        }
    }

    image
}

pub(crate) fn generate_image(mut images: ResMut<Assets<Image>>, tile_side: u32) -> Handle<Image> {
    let image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = make_rhombus_tile(
        UVec2::new(tile_side * 2, tile_side),
        image::Rgba([255, 255, 255, 255]),
        0,
        image::Rgba([255, 255, 255, 255]),
    );
    // This does 3 things in one line:
    // 1. Create a DynamicImage from our ImageBuffer
    // 2. Convert that to an ImageBuffer<Rgba<u8>, _>
    // 3. Convert it back into a DynamicImage
    // You can skip steps 2 and 3 here if your image is built in Rgba<u8>
    let dynamic_image = DynamicImage::from(image).to_rgba8().into();

    // Now add it to Bevy!
    return images.add(Image::from_dynamic(dynamic_image, true));
    // Then spawn the sprite, or whatever else you'd like to do with it
}
