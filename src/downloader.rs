extern crate reqwest;

use base64::encode;
use error::Result;
use std::fs;
use std::path::PathBuf;

pub fn fetch_image(url: &str, download_dir: &PathBuf) -> Result<String> {
    let image_name = encode(url);
    let download_name = download_dir.join(&image_name);
    let mut image = reqwest::get(url)?;
    let mut buffer: Vec<u8> = vec![];

    image.copy_to(&mut buffer)?;

    fs::write(&download_name, buffer)?;

    return Ok(image_name);
}

#[cfg(test)]
mod tests {
    extern crate image;

    use super::*;
    use std::path::Path;

    #[test]
    fn test_it_downloads_an_image_from_a_url() {
        let img_url =
            "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";

        let directory = PathBuf::from("/tmp");
        let img_file_name = fetch_image(&img_url, &directory).expect("failed to download file");

        assert!(Path::new(&directory).join(&img_file_name).exists());
    }

    #[test]
    fn test_it_can_open_the_saved_image() {
        use std::fs::File;
        use std::io::prelude::*;

        let img_url =
            "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png";
        let directory = PathBuf::from("/tmp");

        let img_file_name = fetch_image(&img_url, &directory).expect("failed to download file");

        let mut f = File::open(Path::new(&directory).join(&img_file_name)).unwrap();
        let mut buffer = [0; 10];
        f.read_exact(&mut buffer)
            .expect("Failed to read bytes into test buffer");

        assert_eq!(
            image::guess_format(&buffer).expect("failed to read buffer"),
            image::ImageFormat::PNG
        );
    }
}
