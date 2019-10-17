use image::{Rgba, DynamicImage, RgbaImage};

pub struct DisplayedMap {
    pub title: String,
    pub image: DynamicImage,
}

impl DisplayedMap {
    pub fn new(width: u16, height: u16) -> Self {
        use image::Pixel;

        Self {
            title: format!("test map - rgba {}x{}", width, height),
            image: DynamicImage::ImageRgba8(RgbaImage::from_pixel(
                width as u32,
                height as u32,
                Pixel::from_channels(0, 0, 0, 255),
            )),
        }
    }

    pub fn update(&mut self, counter: u32) -> bool {
        let img = match &mut self.image {
            DynamicImage::ImageRgba8(i) => i,
            _ => unreachable!(),
        };

	let c = counter as u8;

        for (x, y, p) in img.enumerate_pixels_mut() {
            let r = (c / 31).wrapping_add((25 + 191*x + 7*x*x + 17*x*y + 9*y*y) as u8);
            let g = (c / 63).wrapping_add((7 + 5*x*x + 3*x*y + 11*x*y*y + 7*y) as u8);
            let b = (c / 19).wrapping_add((13 + 3*x + 2*y + 7*y*y) as u8);

            *p = Rgba([r, g, b, 255]);
        }

        true
    }
}
