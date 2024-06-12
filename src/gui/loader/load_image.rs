use nannou::image::DynamicImage;

pub fn load_image(filename: &str) -> DynamicImage {
    if let Ok(image) = nannou::image::open(filename) {
        println!("Image loaded successfully");
        return image;
    }
    panic!("Failed to load image: {}", filename);
}

#[cfg(test)]
mod tests {
    use nannou::image::GenericImageView;

    use super::*;

    #[test]
    fn test_load_image() {
        let filename = "assets/images/pacman.jpeg";

        let image = load_image(filename);
        assert_eq!(image.dimensions(), (225, 225)); 
    }
}